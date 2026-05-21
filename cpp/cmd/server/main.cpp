#include "config.hpp"
#include "model.hpp"

#include <httplib.h>
#include <nlohmann/json.hpp>

#include <cassandra.h>
#include <ctime>

#include <array>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

using json = nlohmann::json;

// ---------------------------------------------------------------------------
// Low-level helpers
// ---------------------------------------------------------------------------

// Execute a bound statement and return the result (caller must cass_result_free).
static const CassResult* execute_query(CassSession* session, CassStatement* stmt) {
    CassFuture* f = cass_session_execute(session, stmt);
    cass_future_wait(f);
    CassError rc = cass_future_error_code(f);
    if (rc != CASS_OK) {
        const char* msg; size_t len;
        cass_future_error_message(f, &msg, &len);
        std::string err(msg, len);
        cass_future_free(f);
        throw std::runtime_error("query failed: " + err);
    }
    const CassResult* result = cass_future_get_result(f);
    cass_future_free(f);
    return result;
}

// Execute a statement without collecting results (fire-and-forget, logs errors).
static void execute_void(CassSession* session, CassStatement* stmt) {
    CassFuture* f = cass_session_execute(session, stmt);
    cass_statement_free(stmt);
    cass_future_wait(f);
    CassError rc = cass_future_error_code(f);
    if (rc != CASS_OK) {
        const char* msg; size_t len;
        cass_future_error_message(f, &msg, &len);
        std::cerr << "Warning: " << std::string(msg, len) << "\n";
    }
    cass_future_free(f);
}

static CassUuid parse_uuid(const std::string& s) {
    CassUuid uuid;
    if (cass_uuid_from_string(s.c_str(), &uuid) != CASS_OK)
        throw std::runtime_error("invalid UUID: " + s);
    return uuid;
}

// "YYYY-MM-DDThh:mm:ssZ" or "YYYY-MM-DD" → milliseconds since Unix epoch.
static int64_t parse_timestamp_ms(const std::string& s) {
    struct tm tm = {};
    const char* end = strptime(s.c_str(), "%Y-%m-%dT%H:%M:%SZ", &tm);
    if (!end) end = strptime(s.c_str(), "%Y-%m-%dT%H:%M:%S", &tm);
    if (!end) end = strptime(s.c_str(), "%Y-%m-%d", &tm);
    if (!end) throw std::runtime_error("invalid timestamp: " + s);
    return static_cast<int64_t>(timegm(&tm)) * 1000LL;
}

// "YYYY-MM-DD" → days since Unix epoch (CQL DATE representation).
static uint32_t parse_date_days(const std::string& s) {
    struct tm tm = {};
    const char* end = strptime(s.c_str(), "%Y-%m-%d", &tm);
    if (!end) throw std::runtime_error("invalid date: " + s);
    return static_cast<uint32_t>(timegm(&tm) / 86400LL);
}

// ---------------------------------------------------------------------------
// Row deserialization helpers
// ---------------------------------------------------------------------------

static std::string col_str(const CassRow* row, size_t idx) {
    const char* v; size_t len;
    cass_value_get_string(cass_row_get_column(row, idx), &v, &len);
    return std::string(v, len);
}

// ---------------------------------------------------------------------------
// Route handlers (lambdas capture session by reference)
// ---------------------------------------------------------------------------

// GET /api/owner/{id}
static void handle_owner(CassSession* session,
                          const httplib::Request& req,
                          httplib::Response& res) {
    CassUuid owner_id = parse_uuid(std::string(req.matches[1]));

    const char* q =
        "SELECT owner_id, address, name FROM carepet.owner WHERE owner_id = ?";
    CassStatement* stmt = cass_statement_new(q, 1);
    cass_statement_bind_uuid(stmt, 0, owner_id);
    const CassResult* result = execute_query(session, stmt);
    cass_statement_free(stmt);

    if (cass_result_row_count(result) == 0) {
        cass_result_free(result);
        res.status = 404;
        res.set_content("owner not found", "text/plain");
        return;
    }

    const CassRow* row = cass_result_first_row(result);
    Owner o;
    cass_value_get_uuid(cass_row_get_column(row, 0), &o.owner_id);
    o.address = col_str(row, 1);
    o.name    = col_str(row, 2);
    cass_result_free(result);

    json j;
    j["owner_id"] = uuid_to_string(o.owner_id);
    j["address"]  = o.address;
    j["name"]     = o.name;
    res.set_content(j.dump(), "application/json");
}

// GET /api/owner/{id}/pets
static void handle_owner_pets(CassSession* session,
                               const httplib::Request& req,
                               httplib::Response& res) {
    CassUuid owner_id = parse_uuid(std::string(req.matches[1]));

    const char* q =
        "SELECT owner_id, pet_id, chip_id, species, breed, color, gender,"
        " age, weight, address, name"
        " FROM carepet.pet WHERE owner_id = ?";
    CassStatement* stmt = cass_statement_new(q, 1);
    cass_statement_bind_uuid(stmt, 0, owner_id);
    const CassResult* result = execute_query(session, stmt);
    cass_statement_free(stmt);

    json arr = json::array();
    CassIterator* it = cass_iterator_from_result(result);
    while (cass_iterator_next(it)) {
        const CassRow* row = cass_iterator_get_row(it);
        Pet p;
        cass_value_get_uuid(cass_row_get_column(row, 0), &p.owner_id);
        cass_value_get_uuid(cass_row_get_column(row, 1), &p.pet_id);
        p.chip_id = col_str(row, 2);
        p.species = col_str(row, 3);
        p.breed   = col_str(row, 4);
        p.color   = col_str(row, 5);
        p.gender  = col_str(row, 6);
        cass_value_get_int32(cass_row_get_column(row, 7),
                             reinterpret_cast<cass_int32_t*>(&p.age));
        cass_value_get_float(cass_row_get_column(row, 8), &p.weight);
        p.address = col_str(row, 9);
        p.name    = col_str(row, 10);

        json jp;
        jp["owner_id"] = uuid_to_string(p.owner_id);
        jp["pet_id"]   = uuid_to_string(p.pet_id);
        jp["chip_id"]  = p.chip_id;
        jp["species"]  = p.species;
        jp["breed"]    = p.breed;
        jp["color"]    = p.color;
        jp["gender"]   = p.gender;
        jp["age"]      = p.age;
        jp["weight"]   = p.weight;
        jp["address"]  = p.address;
        jp["name"]     = p.name;
        arr.push_back(jp);
    }
    cass_iterator_free(it);
    cass_result_free(result);
    res.set_content(arr.dump(), "application/json");
}

// GET /api/pet/{id}/sensors
static void handle_pet_sensors(CassSession* session,
                                const httplib::Request& req,
                                httplib::Response& res) {
    CassUuid pet_id = parse_uuid(std::string(req.matches[1]));

    const char* q =
        "SELECT pet_id, sensor_id, type FROM carepet.sensor WHERE pet_id = ?";
    CassStatement* stmt = cass_statement_new(q, 1);
    cass_statement_bind_uuid(stmt, 0, pet_id);
    const CassResult* result = execute_query(session, stmt);
    cass_statement_free(stmt);

    json arr = json::array();
    CassIterator* it = cass_iterator_from_result(result);
    while (cass_iterator_next(it)) {
        const CassRow* row = cass_iterator_get_row(it);
        Sensor s;
        cass_value_get_uuid(cass_row_get_column(row, 0), &s.pet_id);
        cass_value_get_uuid(cass_row_get_column(row, 1), &s.sensor_id);
        s.type = col_str(row, 2);

        json js;
        js["pet_id"]    = uuid_to_string(s.pet_id);
        js["sensor_id"] = uuid_to_string(s.sensor_id);
        js["type"]      = s.type;
        arr.push_back(js);
    }
    cass_iterator_free(it);
    cass_result_free(result);
    res.set_content(arr.dump(), "application/json");
}

// GET /api/sensor/{id}/values?from=ISO8601&to=ISO8601  → array of floats
static void handle_sensor_values(CassSession* session,
                                  const httplib::Request& req,
                                  httplib::Response& res) {
    if (!req.has_param("from") || !req.has_param("to")) {
        res.status = 400;
        res.set_content("missing 'from' and/or 'to' query parameters", "text/plain");
        return;
    }

    CassUuid sensor_id = parse_uuid(std::string(req.matches[1]));
    int64_t from_ms    = parse_timestamp_ms(req.get_param_value("from"));
    int64_t to_ms      = parse_timestamp_ms(req.get_param_value("to"));

    const char* q =
        "SELECT value FROM carepet.measurement"
        " WHERE sensor_id = ? AND ts >= ? AND ts <= ?";
    CassStatement* stmt = cass_statement_new(q, 3);
    cass_statement_bind_uuid(stmt,  0, sensor_id);
    cass_statement_bind_int64(stmt, 1, from_ms);
    cass_statement_bind_int64(stmt, 2, to_ms);
    const CassResult* result = execute_query(session, stmt);
    cass_statement_free(stmt);

    json arr = json::array();
    CassIterator* it = cass_iterator_from_result(result);
    while (cass_iterator_next(it)) {
        cass_float_t v;
        cass_value_get_float(cass_row_get_column(cass_iterator_get_row(it), 0), &v);
        arr.push_back(static_cast<float>(v));
    }
    cass_iterator_free(it);
    cass_result_free(result);
    res.set_content(arr.dump(), "application/json");
}

// GET /api/sensor/{id}/values/day/{YYYY-MM-DD}  → array of 24 hourly avg floats
static void handle_sensor_day(CassSession* session,
                               const httplib::Request& req,
                               httplib::Response& res) {
    CassUuid sensor_id = parse_uuid(std::string(req.matches[1]));
    std::string date_str = req.matches[2];
    uint32_t date_days   = parse_date_days(date_str);

    std::array<float, 24> avgs   = {};
    std::array<int,   24> counts = {};

    // Check for pre-computed hourly averages.
    {
        const char* q =
            "SELECT hour, value FROM carepet.sensor_avg"
            " WHERE sensor_id = ? AND date = ?";
        CassStatement* stmt = cass_statement_new(q, 2);
        cass_statement_bind_uuid(stmt,   0, sensor_id);
        cass_statement_bind_uint32(stmt, 1, date_days);
        const CassResult* result = execute_query(session, stmt);
        cass_statement_free(stmt);

        bool has_avg = cass_result_row_count(result) > 0;
        if (has_avg) {
            CassIterator* it = cass_iterator_from_result(result);
            while (cass_iterator_next(it)) {
                const CassRow* row = cass_iterator_get_row(it);
                cass_int32_t hour;
                cass_float_t value;
                cass_value_get_int32(cass_row_get_column(row, 0), &hour);
                cass_value_get_float(cass_row_get_column(row, 1), &value);
                if (hour >= 0 && hour < 24) avgs[hour] = value;
            }
            cass_iterator_free(it);
            cass_result_free(result);

            json arr = json::array();
            for (float v : avgs) arr.push_back(v);
            res.set_content(arr.dump(), "application/json");
            return;
        }
        cass_result_free(result);
    }

    // No pre-computed data: compute from raw measurements.
    int64_t day_start_ms = static_cast<int64_t>(date_days) * 86400LL * 1000LL;
    int64_t day_end_ms   = day_start_ms + 86400LL * 1000LL;

    {
        const char* q =
            "SELECT ts, value FROM carepet.measurement"
            " WHERE sensor_id = ? AND ts >= ? AND ts < ?";
        CassStatement* stmt = cass_statement_new(q, 3);
        cass_statement_bind_uuid(stmt,  0, sensor_id);
        cass_statement_bind_int64(stmt, 1, day_start_ms);
        cass_statement_bind_int64(stmt, 2, day_end_ms);
        const CassResult* result = execute_query(session, stmt);
        cass_statement_free(stmt);

        CassIterator* it = cass_iterator_from_result(result);
        while (cass_iterator_next(it)) {
            const CassRow* row = cass_iterator_get_row(it);
            cass_int64_t ts;
            cass_float_t value;
            cass_value_get_int64(cass_row_get_column(row, 0), &ts);
            cass_value_get_float(cass_row_get_column(row, 1), &value);
            int hour = static_cast<int>((ts - day_start_ms) / (3600LL * 1000LL));
            if (hour >= 0 && hour < 24) {
                avgs[hour] += value;
                counts[hour]++;
            }
        }
        cass_iterator_free(it);
        cass_result_free(result);
    }

    // Compute averages and persist non-empty hours.
    const char* iq =
        "INSERT INTO carepet.sensor_avg (sensor_id, date, hour, value) VALUES (?, ?, ?, ?)";
    for (int h = 0; h < 24; ++h) {
        if (counts[h] > 0) {
            avgs[h] /= static_cast<float>(counts[h]);
            CassStatement* istmt = cass_statement_new(iq, 4);
            cass_statement_bind_uuid(istmt,   0, sensor_id);
            cass_statement_bind_uint32(istmt, 1, date_days);
            cass_statement_bind_int32(istmt,  2, static_cast<cass_int32_t>(h));
            cass_statement_bind_float(istmt,  3, avgs[h]);
            execute_void(session, istmt);
        }
    }

    json arr = json::array();
    for (float v : avgs) arr.push_back(v);
    res.set_content(arr.dump(), "application/json");
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------
int main(int argc, char** argv) {
    try {
        Config cfg;
        cfg.parse_args(argc, argv);

        int port = 8000;
        for (int i = 1; i + 1 < argc; ++i) {
            std::string a = argv[i];
            while (!a.empty() && a.front() == '-') a.erase(0, 1);
            if (a == "port") port = std::stoi(argv[i + 1]);
        }

        std::cout << "Connecting to " << cfg.hosts << " ...\n";
        Session session;
        session.connect_keyspace(cfg, "carepet");
        CassSession* raw = session.get();

        httplib::Server svr;

        // Register more-specific paths before less-specific ones.
        svr.Get(R"(/api/owner/([^/]+)/pets)",
            [raw](const httplib::Request& req, httplib::Response& res) {
                try { handle_owner_pets(raw, req, res); }
                catch (const std::exception& e) {
                    res.status = 500;
                    res.set_content(e.what(), "text/plain");
                }
            });

        svr.Get(R"(/api/owner/([^/]+))",
            [raw](const httplib::Request& req, httplib::Response& res) {
                try { handle_owner(raw, req, res); }
                catch (const std::exception& e) {
                    res.status = 500;
                    res.set_content(e.what(), "text/plain");
                }
            });

        svr.Get(R"(/api/pet/([^/]+)/sensors)",
            [raw](const httplib::Request& req, httplib::Response& res) {
                try { handle_pet_sensors(raw, req, res); }
                catch (const std::exception& e) {
                    res.status = 500;
                    res.set_content(e.what(), "text/plain");
                }
            });

        svr.Get(R"(/api/sensor/([^/]+)/values/day/([^/]+))",
            [raw](const httplib::Request& req, httplib::Response& res) {
                try { handle_sensor_day(raw, req, res); }
                catch (const std::exception& e) {
                    res.status = 500;
                    res.set_content(e.what(), "text/plain");
                }
            });

        svr.Get(R"(/api/sensor/([^/]+)/values)",
            [raw](const httplib::Request& req, httplib::Response& res) {
                try { handle_sensor_values(raw, req, res); }
                catch (const std::exception& e) {
                    res.status = 500;
                    res.set_content(e.what(), "text/plain");
                }
            });

        std::cout << "Server listening on 0.0.0.0:" << port << "\n";
        svr.listen("0.0.0.0", port);

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }
    return 0;
}
