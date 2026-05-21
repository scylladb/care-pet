## Build an IoT App with C++

### Architecture

This section walks through and explains the code for the different commands.
As explained in the Getting Started page, the project is structured as follows:

- migrate (`/cmd/migrate/main.cpp`) – creates the `carepet` keyspace and tables
- collar (`/cmd/sensor/main.cpp`) – generates pet health data and pushes it into the storage
- web app (`/cmd/server/main.cpp`) – REST API service for tracking pets' health state

### Migrate

Start by creating a local ScyllaDB cluster consisting of 3 nodes:

```
$ docker-compose up -d
```

Docker-compose will spin up a ScyllaDB cluster consisting of 3 nodes
(`carepet-scylla1`, `carepet-scylla2` and `carepet-scylla3`) along with
the app container.  Wait for about two minutes and check the status of the
cluster:

```
$ docker exec -it carepet-scylla1 nodetool status
```

Once all nodes show `UN` (Up Normal) status, run the migrate command.
First, get the node IP address:

```
$ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1
```

Then run the migrate binary:

```
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
$ docker exec carepet-cpp migrate --hosts $NODE1
```

The command executes `cmd/migrate/main.cpp`. It creates the keyspace and
tables needed by the collar and server services.

The `main` function in `cmd/migrate/main.cpp` connects first without a
keyspace to create it, then reconnects with the keyspace to create all tables:

```cpp
// cmd/migrate/main.cpp

int main(int argc, char** argv) {
    Config cfg;
    cfg.parse_args(argc, argv);

    // Step 1: connect without a keyspace and create it.
    {
        Session s;
        s.connect(cfg);
        s.execute(KEYSPACE_DDL);
    }

    // Step 2: connect to the keyspace and create all tables.
    {
        Session s;
        s.connect_keyspace(cfg, "carepet");
        for (auto& [name, ddl] : TABLE_DDLS) {
            s.execute(ddl);
        }
    }
}
```

The `Session` class in `include/config.hpp` is a RAII wrapper around
`CassCluster` and `CassSession` from the ScyllaDB C++ Driver.  It throws
a `std::runtime_error` on any failure:

```cpp
// include/config.hpp

void connect(const Config& cfg) {
    apply_config(cfg);
    check(cass_session_connect(session_, cluster_), "connect");
}

void connect_keyspace(const Config& cfg, const std::string& ks) {
    apply_config(cfg);
    check(cass_session_connect_keyspace(session_, cluster_, ks.c_str()),
          "connect_keyspace(" + ks + ")");
}
```

`apply_config` sets the contact points and credentials:

```cpp
void apply_config(const Config& cfg) {
    cass_cluster_set_contact_points(cluster_, cfg.hosts.c_str());
    cass_cluster_set_port(cluster_, cfg.port);
    if (!cfg.username.empty())
        cass_cluster_set_credentials(cluster_,
                                     cfg.username.c_str(),
                                     cfg.password.c_str());
}
```

The keyspace DDL creates a keyspace with `NetworkTopologyStrategy` and
replication factor 3:

```sql
CREATE KEYSPACE IF NOT EXISTS carepet
    WITH replication = {'class':'NetworkTopologyStrategy','replication_factor':'3'}
    AND durable_writes = TRUE
```

You can verify the database structure with:

```
$ docker exec -it carepet-scylla1 cqlsh
cqlsh> USE carepet;
cqlsh:carepet> DESCRIBE TABLES
cqlsh:carepet> DESCRIBE TABLE pet
```

Expected output:

```
CREATE TABLE carepet.pet (
    owner_id uuid,
    pet_id uuid,
    chip_id text,
    species text,
    breed text,
    color text,
    gender text,
    address text,
    age int,
    name text,
    weight float,
    PRIMARY KEY (owner_id, pet_id)
) WITH CLUSTERING ORDER BY (pet_id ASC)
    ...
```

### Sensor

The sensor service simulates the collar's activity and periodically saves
measurements to the database.  Run it with:

```
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
$ docker exec -d carepet-cpp sensor --hosts $NODE1 --measure 5s --buffer-interval 1m
```

Arguments:

- `--hosts` – contact point(s) for ScyllaDB
- `--measure` – interval between sensor samples (e.g. `5s`, `1m`, `2h`)
- `--buffer-interval` – how often buffered measurements are flushed to the DB

The `main` function in `cmd/sensor/main.cpp` generates random owner, pet,
and sensors, persists them, then enters the measurement loop:

```cpp
// cmd/sensor/main.cpp

int main(int argc, char** argv) {
    Config cfg;
    cfg.parse_args(argc, argv);
    // ... parse --measure and --buffer-interval ...

    Session session;
    session.connect_keyspace(cfg, "carepet");

    CassUuidGen* uuid_gen = cass_uuid_gen_new();
    Owner owner = rand_owner(uuid_gen);
    Pet   pet   = rand_pet(uuid_gen, owner);

    int num_sensors = rand_int(2, 4);
    std::vector<Sensor> sensors;
    for (int i = 0; i < num_sensors; ++i)
        sensors.push_back(rand_sensor(uuid_gen, pet));

    insert_owner(session.get(), owner);
    insert_pet(session.get(), pet);
    for (const auto& s : sensors)
        insert_sensor(session.get(), s);

    // Prepare the measurement INSERT once for efficiency.
    const CassPrepared* prepared = /* prepare INSERT INTO measurement ... */;

    std::vector<Measurement> buffer;
    while (true) {
        // Sample sensors every --measure interval.
        for (const auto& s : sensors) {
            Measurement m;
            m.sensor_id = s.sensor_id;
            m.ts        = now_ms();       // milliseconds since Unix epoch
            m.value     = rand_sensor_data(s);
            buffer.push_back(m);
        }

        // Flush to ScyllaDB every --buffer-interval.
        flush_measurements(session.get(), prepared, buffer);
        buffer.clear();
    }
}
```

The measurement `INSERT` is prepared once using `cass_session_prepare` to
avoid repeated query-planning overhead:

```cpp
// cmd/sensor/main.cpp

const char* mq =
    "INSERT INTO carepet.measurement (sensor_id, ts, value) VALUES (?, ?, ?)";
CassFuture* pf = cass_session_prepare(session.get(), mq);
cass_future_wait(pf);
const CassPrepared* prepared = cass_future_get_prepared(pf);
cass_future_free(pf);
```

`rand_sensor_data` in `include/rand_util.hpp` generates realistic values
for each sensor type:

```cpp
// include/rand_util.hpp

inline float rand_sensor_data(const Sensor& s) {
    if (s.type == "T") return rand_float(98.0f, 107.0f);  // temperature °F
    if (s.type == "P") return rand_float(80.0f, 120.0f);  // pulse bpm
    if (s.type == "R") return rand_float(33.0f,  37.0f);  // respiration /min
    return rand_float(0.0f, 10.0f);                        // location distance
}
```

Expected output:

```
New owner # 3d6c7e2a-f1c0-4b8e-9d1a-000000000001
New pet    # a1b2c3d4-0000-0000-0000-000000000002
  Sensor [T] # 11111111-0000-0000-0000-000000000001
  Sensor [P] # 22222222-0000-0000-0000-000000000002
Sensor loop started (measure every 5s, flush every 60s) ...
[T] 11111111-... = 101.34
[P] 22222222-... = 97.12
...
Flushing 24 measurement(s) to ScyllaDB ...
```

Write down the owner ID (the UUID after `New owner #`).

### Server

The server service is a REST API for querying pet health data.  It is built
with [cpp-httplib](https://github.com/yhirose/cpp-httplib) and
[nlohmann/json](https://github.com/nlohmann/json).

Run it with:

```
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
$ docker exec -d carepet-cpp server --hosts $NODE1 --port 8000
```

The `main` function in `cmd/server/main.cpp` connects to the `carepet`
keyspace and registers route handlers on a `httplib::Server`:

```cpp
// cmd/server/main.cpp

int main(int argc, char** argv) {
    Config cfg;
    cfg.parse_args(argc, argv);
    int port = 8000;
    // ... parse --port ...

    Session session;
    session.connect_keyspace(cfg, "carepet");
    CassSession* raw = session.get();

    httplib::Server svr;

    svr.Get(R"(/api/owner/([^/]+)/pets)",    [raw](...) { handle_owner_pets(raw, ...); });
    svr.Get(R"(/api/owner/([^/]+))",         [raw](...) { handle_owner(raw, ...); });
    svr.Get(R"(/api/pet/([^/]+)/sensors)",   [raw](...) { handle_pet_sensors(raw, ...); });
    svr.Get(R"(/api/sensor/([^/]+)/values/day/([^/]+))",
                                             [raw](...) { handle_sensor_day(raw, ...); });
    svr.Get(R"(/api/sensor/([^/]+)/values)", [raw](...) { handle_sensor_values(raw, ...); });

    svr.listen("0.0.0.0", port);
}
```

The `handle_owner` function queries the `owner` table and returns JSON:

```cpp
// cmd/server/main.cpp

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
        res.status = 404;
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
```

The `handle_sensor_day` handler implements a lazy-evaluated hourly average
cache: it checks `sensor_avg` first; if no data is found it reads raw
measurements, computes hourly averages, stores them in `sensor_avg`, and
returns the 24-element array:

```cpp
// cmd/server/main.cpp

static void handle_sensor_day(CassSession* session, ...) {
    // 1. Check for pre-computed averages.
    // SELECT hour, value FROM carepet.sensor_avg WHERE sensor_id = ? AND date = ?
    if (has_avg) {
        // return cached averages immediately
        return;
    }

    // 2. Compute from raw measurements.
    // SELECT ts, value FROM carepet.measurement WHERE sensor_id = ? AND ts >= ? AND ts < ?
    for each measurement {
        int hour = (ts - day_start_ms) / (3600 * 1000);
        avgs[hour] += value;
        counts[hour]++;
    }

    // 3. Store computed averages back into sensor_avg.
    // INSERT INTO carepet.sensor_avg (sensor_id, date, hour, value) VALUES (?, ?, ?, ?)

    // 4. Return the 24-element JSON array.
}
```

### REST API Reference

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/owner/{uuid}` | Fetch owner by ID |
| GET | `/api/owner/{uuid}/pets` | List pets for an owner |
| GET | `/api/pet/{uuid}/sensors` | List sensors for a pet |
| GET | `/api/sensor/{uuid}/values?from=T&to=T` | Raw readings in time range |
| GET | `/api/sensor/{uuid}/values/day/{YYYY-MM-DD}` | 24 hourly averages |

Use the owner ID printed by the sensor simulator to test the API:

```bash
# Get owner
curl http://127.0.0.1:8000/api/owner/{owner_id}
# → {"address":"...","name":"...","owner_id":"..."}

# List pets
curl http://127.0.0.1:8000/api/owner/{owner_id}/pets
# → [{"age":6,"name":"...","owner_id":"...","pet_id":"...","weight":41.7,...}]

# List sensors
curl http://127.0.0.1:8000/api/pet/{pet_id}/sensors
# → [{"pet_id":"...","sensor_id":"...","type":"T"},...]

# Raw readings in a time range (ISO 8601 timestamps)
curl "http://127.0.0.1:8000/api/sensor/{sensor_id}/values?from=2024-01-15T00:00:00Z&to=2024-01-15T23:59:59Z"
# → [101.34,99.87,...]

# Hourly averages for a day
curl http://127.0.0.1:8000/api/sensor/{sensor_id}/values/day/2024-01-15
# → [0.0,0.0,...,101.1,...,0.0]   (24 floats, one per hour)
```
