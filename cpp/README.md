# Care-Pet C++

ScyllaDB IoT demo application written in C++17 using the
[ScyllaDB C++ Driver](https://github.com/scylladb/cpp-driver).  
It models a pet health monitoring collar that streams sensor readings
(temperature, pulse, respiration, location) into ScyllaDB and exposes
them through a REST API.

```
┌──────────────┐   INSERT measurements    ┌──────────────────┐
│    sensor    │ ─────────────────────────▶│                  │
│ (collar sim) │                           │    ScyllaDB      │
└──────────────┘                           │    cluster       │
                                           │                  │
┌──────────────┐   SELECT / INSERT avgs   │                  │
│    server    │◀─────────────────────────▶│                  │
│  (REST API)  │                           └──────────────────┘
└──────────────┘
        ▲
        │  HTTP/JSON
        ▼
   curl / client
```

---

## Prerequisites

| Tool | Version |
|------|---------|
| CMake | ≥ 3.14 |
| C++ compiler | GCC ≥ 9 or Clang ≥ 10 (C++17) |
| libuv | ≥ 1.x |
| OpenSSL | ≥ 1.1 |
| Docker + docker-compose | any recent version |

> **ScyllaDB C++ Driver** is fetched and built automatically inside the
> Docker image.  For a local build install the driver first – see
> [driver installation](https://github.com/scylladb/cpp-driver#installation).

---

## Quick Start (Docker)

```bash
# 1. Start a 3-node ScyllaDB cluster + build the app image
docker-compose up -d

# 2. Wait for ScyllaDB to be ready (≈ 60 s)
docker exec carepet-scylla1 nodetool status

# 3. Run migrate inside the app container
docker exec carepet-cpp migrate --hosts carepet-scylla1

# 4. Start the collar simulator (background)
docker exec -d carepet-cpp sensor --hosts carepet-scylla1

# 5. Start the REST server
docker exec -d carepet-cpp server --hosts carepet-scylla1 --port 8000
```

---

## Local Build

```bash
# Install driver (Ubuntu/Debian example)
# See https://github.com/scylladb/cpp-driver for your platform.

git clone https://github.com/scylladb/cpp-driver.git
cmake -B cpp-driver/build -S cpp-driver -DCMAKE_BUILD_TYPE=Release \
      -DCASS_BUILD_SHARED=ON -DCASS_BUILD_STATIC=OFF
cmake --build cpp-driver/build --parallel
sudo cmake --install cpp-driver/build

# Build care-pet binaries
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel
```

The three binaries are placed in `build/`:

```
build/migrate
build/sensor
build/server
```

---

## Running

### 1. Migrate

Creates the `carepet` keyspace and all tables.

```bash
./build/migrate [--hosts 127.0.0.1] [--username u] [--password p]
```

```
Connecting to 127.0.0.1 ...
Creating keyspace 'carepet' ...
  OK
Creating table 'owner' ...
  OK
...
Migration complete.
```

### 2. Sensor Simulator

Generates a random owner + pet + 2–4 sensors, inserts them, then streams
measurements in a loop.

```bash
./build/sensor \
  [--hosts 127.0.0.1] \
  [--measure 5s]          # how often to sample (s/m/h)
  [--buffer-interval 1m]  # how often to flush to DB
```

```
New owner # 3d6c7e2a-...
New pet    # a1b2c3d4-...
  Sensor [T] # 11111111-...
  Sensor [P] # 22222222-...
Sensor loop started (measure every 5s, flush every 60s) ...
[T] 11111111-... = 101.34
[P] 22222222-... = 97.12
...
Flushing 24 measurement(s) to ScyllaDB ...
```

Duration strings: `5s` = 5 seconds, `1m` = 60 seconds, `2h` = 7200 seconds.

### 3. Server

```bash
./build/server \
  [--hosts 127.0.0.1] \
  [--port 8000]
```

---

## REST API

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/owner/{uuid}` | Fetch owner by ID |
| GET | `/api/owner/{uuid}/pets` | List pets for an owner |
| GET | `/api/pet/{uuid}/sensors` | List sensors for a pet |
| GET | `/api/sensor/{uuid}/values?from=T&to=T` | Raw readings in time range |
| GET | `/api/sensor/{uuid}/values/day/{YYYY-MM-DD}` | 24 hourly averages |

Timestamps use ISO 8601: `2024-01-15T12:00:00Z`.

### Examples

```bash
# Get owner
curl http://localhost:8000/api/owner/3d6c7e2a-0000-0000-0000-000000000000

# List pets
curl http://localhost:8000/api/owner/3d6c7e2a-.../pets

# List sensors
curl http://localhost:8000/api/pet/a1b2c3d4-.../sensors

# Readings in range
curl "http://localhost:8000/api/sensor/11111111-.../values?from=2024-01-15T00:00:00Z&to=2024-01-15T23:59:59Z"
# → [101.34, 99.87, ...]

# Hourly averages for a day (computed on demand + cached in sensor_avg)
curl http://localhost:8000/api/sensor/11111111-.../values/day/2024-01-15
# → [0,0,0,...,101.1,100.3,...,0]  (24 floats)
```

---

## ScyllaDB Cloud

ScyllaDB Cloud clusters use username/password authentication over standard
CQL (port 9042, no SSL required). Set the connection details via environment
variables or CLI flags:

```bash
export SCYLLADB_HOSTS=node-0.aws-us-east-1.xxx.clusters.scylla.cloud
export SCYLLADB_USERNAME=scylla
export SCYLLADB_PASSWORD=secret
./build/migrate
./build/sensor --measure 5s --buffer-interval 1m
```

Environment variables `SCYLLADB_HOSTS`, `SCYLLADB_USERNAME`, and
`SCYLLADB_PASSWORD` are read first; CLI flags (`--hosts`, `--username`,
`--password`) override them.

---

## Project Structure

```
cpp/
├── CMakeLists.txt          # Build configuration
├── Dockerfile              # Multi-stage Docker build
├── docker-compose.yml      # 3-node ScyllaDB cluster + app
├── db/
│   ├── keyspace.cql        # CREATE KEYSPACE statement
│   └── schema.cql          # CREATE TABLE statements
├── include/
│   ├── config.hpp          # Config struct + RAII Session wrapper
│   ├── model.hpp           # C++ data-model structs
│   └── rand_util.hpp       # Random test-data generators
└── cmd/
    ├── migrate/main.cpp    # Keyspace + schema migration
    ├── sensor/main.cpp     # Collar simulator
    └── server/main.cpp     # REST API server
```

---

## Implementation Notes

* **Driver API** — The ScyllaDB C++ Driver exposes a pure C API (`cassandra.h`).
  `config.hpp` wraps the raw `CassCluster` / `CassSession` pair in a RAII
  `Session` class that throws `std::runtime_error` on failure.

* **Prepared statements** — The sensor simulator prepares the measurement
  `INSERT` once and reuses the `CassPrepared*` for every write, avoiding
  repeated parse/plan overhead.

* **Hourly averages** — The `/values/day/{date}` endpoint first checks
  `sensor_avg`.  If no rows are found it falls back to computing averages
  from raw `measurement` rows, stores the result in `sensor_avg`, then
  returns the 24-element array.

* **Thread safety** — `CassSession` is thread-safe; cpp-httplib dispatches
  requests from a thread pool and each handler creates its own local
  `CassStatement` / `CassResult` objects.

* **CQL DATE** — Stored as `uint32_t` days since the Unix epoch
  (1970-01-01).  `parse_date_days()` in the server converts an ISO date
  string using POSIX `timegm`.

* **CQL TIMESTAMP** — Stored as `int64_t` milliseconds since the Unix
  epoch, matching the C++ `std::chrono::system_clock` representation.
