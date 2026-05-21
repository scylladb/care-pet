#include "config.hpp"
#include "model.hpp"
#include "rand_util.hpp"

#include <cassandra.h>

#include <chrono>
#include <iostream>
#include <string>
#include <thread>
#include <vector>

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

static int64_t now_ms() {
    using namespace std::chrono;
    return duration_cast<milliseconds>(
               system_clock::now().time_since_epoch())
        .count();
}

// Parse simple duration strings like "5s", "1m", "2h" → seconds.
static int parse_duration_secs(const std::string& s) {
    if (s.empty()) return 0;
    char unit = s.back();
    int  val  = std::stoi(s.substr(0, s.size() - 1));
    if (unit == 's') return val;
    if (unit == 'm') return val * 60;
    if (unit == 'h') return val * 3600;
    return std::stoi(s); // no unit → treat as seconds
}

// ---------------------------------------------------------------------------
// DB helpers
// ---------------------------------------------------------------------------

static void exec_stmt(CassSession* session, CassStatement* stmt, const char* ctx) {
    CassFuture* f = cass_session_execute(session, stmt);
    cass_statement_free(stmt);
    cass_future_wait(f);
    CassError rc = cass_future_error_code(f);
    if (rc != CASS_OK) {
        const char* msg; size_t len;
        cass_future_error_message(f, &msg, &len);
        std::string err(msg, len);
        cass_future_free(f);
        throw std::runtime_error(std::string(ctx) + ": " + err);
    }
    cass_future_free(f);
}

static void insert_owner(CassSession* session, const Owner& o) {
    const char* q =
        "INSERT INTO carepet.owner (owner_id, address, name) VALUES (?, ?, ?)";
    CassStatement* stmt = cass_statement_new(q, 3);
    cass_statement_bind_uuid(stmt, 0, o.owner_id);
    cass_statement_bind_string(stmt, 1, o.address.c_str());
    cass_statement_bind_string(stmt, 2, o.name.c_str());
    exec_stmt(session, stmt, "insert_owner");
}

static void insert_pet(CassSession* session, const Pet& p) {
    const char* q =
        "INSERT INTO carepet.pet"
        " (owner_id, pet_id, chip_id, species, breed, color, gender, age, weight, address, name)"
        " VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
    CassStatement* stmt = cass_statement_new(q, 11);
    cass_statement_bind_uuid(stmt,   0, p.owner_id);
    cass_statement_bind_uuid(stmt,   1, p.pet_id);
    cass_statement_bind_string(stmt, 2, p.chip_id.c_str());
    cass_statement_bind_string(stmt, 3, p.species.c_str());
    cass_statement_bind_string(stmt, 4, p.breed.c_str());
    cass_statement_bind_string(stmt, 5, p.color.c_str());
    cass_statement_bind_string(stmt, 6, p.gender.c_str());
    cass_statement_bind_int32(stmt,  7, static_cast<cass_int32_t>(p.age));
    cass_statement_bind_float(stmt,  8, p.weight);
    cass_statement_bind_string(stmt, 9, p.address.c_str());
    cass_statement_bind_string(stmt,10, p.name.c_str());
    exec_stmt(session, stmt, "insert_pet");
}

static void insert_sensor(CassSession* session, const Sensor& s) {
    const char* q =
        "INSERT INTO carepet.sensor (pet_id, sensor_id, type) VALUES (?, ?, ?)";
    CassStatement* stmt = cass_statement_new(q, 3);
    cass_statement_bind_uuid(stmt,   0, s.pet_id);
    cass_statement_bind_uuid(stmt,   1, s.sensor_id);
    cass_statement_bind_string(stmt, 2, s.type.c_str());
    exec_stmt(session, stmt, "insert_sensor");
}

// Flush buffered measurements using a prepared statement.
static void flush_measurements(CassSession* session,
                                const CassPrepared* prep,
                                const std::vector<Measurement>& buf) {
    for (const auto& m : buf) {
        CassStatement* stmt = cass_prepared_bind(prep);
        cass_statement_bind_uuid(stmt,  0, m.sensor_id);
        cass_statement_bind_int64(stmt, 1, m.ts);
        cass_statement_bind_float(stmt, 2, m.value);

        CassFuture* f = cass_session_execute(session, stmt);
        cass_statement_free(stmt);
        cass_future_wait(f);
        CassError rc = cass_future_error_code(f);
        if (rc != CASS_OK) {
            const char* msg; size_t len;
            cass_future_error_message(f, &msg, &len);
            std::cerr << "Warning: insert measurement failed: "
                      << std::string(msg, len) << "\n";
        }
        cass_future_free(f);
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------
int main(int argc, char** argv) {
    try {
        Config cfg;
        cfg.parse_args(argc, argv);

        // Parse sensor-specific flags.
        int measure_secs = 5;
        int buffer_secs  = 60;
        for (int i = 1; i + 1 < argc; ++i) {
            std::string a = argv[i];
            while (!a.empty() && a.front() == '-') a.erase(0, 1);
            if      (a == "measure")         measure_secs = parse_duration_secs(argv[i + 1]);
            else if (a == "buffer-interval") buffer_secs  = parse_duration_secs(argv[i + 1]);
        }

        std::cout << "Connecting to " << cfg.hosts << " ...\n";
        Session session;
        session.connect_keyspace(cfg, "carepet");

        // Generate random owner / pet / sensors.
        CassUuidGen* uuid_gen = cass_uuid_gen_new();
        Owner owner = rand_owner(uuid_gen);
        Pet   pet   = rand_pet(uuid_gen, owner);

        int num_sensors = rand_int(2, 4);
        std::vector<Sensor> sensors;
        sensors.reserve(num_sensors);
        for (int i = 0; i < num_sensors; ++i)
            sensors.push_back(rand_sensor(uuid_gen, pet));

        // Persist initial data.
        insert_owner(session.get(), owner);
        insert_pet(session.get(), pet);
        for (const auto& s : sensors)
            insert_sensor(session.get(), s);

        std::cout << "New owner # " << uuid_to_string(owner.owner_id) << "\n";
        std::cout << "New pet    # " << uuid_to_string(pet.pet_id) << "\n";
        for (const auto& s : sensors)
            std::cout << "  Sensor [" << s.type << "] # "
                      << uuid_to_string(s.sensor_id) << "\n";

        // Prepare measurement INSERT once.
        const char* mq =
            "INSERT INTO carepet.measurement (sensor_id, ts, value) VALUES (?, ?, ?)";
        CassFuture* pf = cass_session_prepare(session.get(), mq);
        cass_future_wait(pf);
        CassError prc = cass_future_error_code(pf);
        if (prc != CASS_OK) {
            const char* msg; size_t len;
            cass_future_error_message(pf, &msg, &len);
            std::string err(msg, len);
            cass_future_free(pf);
            throw std::runtime_error("prepare failed: " + err);
        }
        const CassPrepared* prepared = cass_future_get_prepared(pf);
        cass_future_free(pf);

        using namespace std::chrono;
        auto measure_dur = seconds(measure_secs);
        auto buffer_dur  = seconds(buffer_secs);
        auto last_measure = steady_clock::now();
        auto last_flush   = steady_clock::now();

        std::vector<Measurement> buffer;

        std::cout << "Sensor loop started (measure every " << measure_secs
                  << "s, flush every " << buffer_secs << "s) ...\n";

        while (true) {
            auto now = steady_clock::now();

            if (now - last_measure >= measure_dur) {
                for (const auto& s : sensors) {
                    Measurement m;
                    m.sensor_id = s.sensor_id;
                    m.ts        = now_ms();
                    m.value     = rand_sensor_data(s);
                    buffer.push_back(m);
                    std::cout << "[" << s.type << "] "
                              << uuid_to_string(s.sensor_id)
                              << " = " << m.value << "\n";
                }
                last_measure = now;
            }

            if (now - last_flush >= buffer_dur) {
                if (!buffer.empty()) {
                    std::cout << "Flushing " << buffer.size()
                              << " measurement(s) to ScyllaDB ...\n";
                    flush_measurements(session.get(), prepared, buffer);
                    buffer.clear();
                }
                last_flush = now;
            }

            std::this_thread::sleep_for(milliseconds(100));
        }

        // Unreachable in normal operation; reached only after SIGINT / clean exit.
        cass_prepared_free(prepared);
        cass_uuid_gen_free(uuid_gen);

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }
    return 0;
}
