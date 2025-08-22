Care Pet ScyllaDB IoT example
===

This example project demonstrates a generic IoT use case
for ScyllaDB in C#.

The documentation for this application and guided exercise is [here](../docs).

The application allows tracking of pets health indicators
and consist of three parts:

- migrate (`dotnet run --project CarePet.Migrate.csproj`) - creates the `carepet` keyspace and tables
- collar (`dotnet run --project CarePet.Sensor.csproj`) - generates a pet health data and pushes it into the storage
- web app (`dotnet run --project CarePet.csproj`) - REST API service for tracking pets health state

Quick Start
---

Prerequisites:

- [.NET 8.0 SDK](https://dotnet.microsoft.com/download/dotnet/8.0)
- [docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

To run a local ScyllaDB cluster consisting of three nodes with
the help of `docker` and `docker-compose` execute:

    $ docker-compose up -d

Docker-compose will spin up three nodes: `csharp-carepet-scylla1-1`, `csharp-carepet-scylla2-1`
and `csharp-carepet-scylla3-1`. You can access them with the `docker` command.

To execute CQLSH:

    $ docker exec -it csharp-csharp-carepet-scylla1-1 cqlsh

To execute nodetool:

    $ docker exec -it csharp-carepet-scylla1-1 nodetool status

Shell:

    $ docker exec -it csharp-carepet-scylla1-1 shell

You can inspect any node by means of the `docker inspect` command
as follows. for example:

    $ docker inspect csharp-carepet-scylla1-1

To get node IP address run:

    $ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' csharp-carepet-scylla1-1

You will need to reference this value multiple times later so if it's easier
for you can save it as a variable `NODE1`:

    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' csharp-carepet-scylla1-1)

To initialize database execute:

    $ dotnet build CarePet.Migrate.csproj
    $ dotnet run --project CarePet.Migrate.csproj --hosts $NODE1

Expected output:

    info: CarePet.Commands.MigrateCommand[0]
          Creating keyspace carepet...
    info: CarePet.Commands.MigrateCommand[0]
          Keyspace carepet created successfully
    info: CarePet.Commands.MigrateCommand[0]
          Creating tables...
    Keyspace: carepet; Table: measurement
    Keyspace: carepet; Table: owner
    Keyspace: carepet; Table: pet
    Keyspace: carepet; Table: sensor
    Keyspace: carepet; Table: sensor_avg

You can check the database structure with:

    $ docker exec -it csharp-carepet-scylla1-1 cqlsh
    cqlsh> DESCRIBE KEYSPACES

    carepet  system_schema  system_auth  system  system_distributed  system_traces

    cqlsh> USE carepet;
    cqlsh:carepet> DESCRIBE TABLES

    pet  sensor_avg  measurement  owner  sensor

    cqlsh:carepet> DESCRIBE TABLE pet

    CREATE TABLE carepet.pet (
        owner_id uuid,
        pet_id   uuid,
        chip_id  text,
        species  text,
        breed    text,
        color    text,
        gender   text,
        address  text,
        age int,
        name text,
        weight float,
        PRIMARY KEY (owner_id, pet_id)
    ) WITH CLUSTERING ORDER BY (pet_id ASC)
        AND bloom_filter_fp_chance = 0.01
        AND caching = {'keys': 'ALL', 'rows_per_partition': 'ALL'}
        AND comment = ''
        AND compaction = {'class': 'SizeTieredCompactionStrategy'}
        AND compression = {'sstable_compression': 'org.apache.cassandra.io.compress.LZ4Compressor'}
        AND crc_check_chance = 1.0
        AND dclocal_read_repair_chance = 0.1
        AND default_time_to_live = 0
        AND gc_grace_seconds = 864000
        AND max_index_interval = 2048
        AND min_index_interval = 128
        AND read_repair_chance = 0.0
        AND speculative_retry = '99.0PERCENTILE';

To generate pet health data:

    $ dotnet build CarePet.Sensor.csproj
    $ dotnet run --project CarePet.Sensor.csproj --hosts $NODE1 --datacenter datacenter1 --measure 00:01:00 --buffer-interval 00:01:00

Expected output:

    info: CarePet.Sensor[0]
      owner = Owner{ownerId=c94a6726-85d4-4c05-b5a8-351837c26c42, name='N9fhYPFp', address='Hu6YX0yTyk'}
    info: CarePet.Sensor[0]
        pet = Pet{ownerId=c94a6726-85d4-4c05-b5a8-351837c26c42, petId=6db45498-244c-460c-a567-2098ab7fb848, chipId='', species='', breed='', color='', gender='', age=63, weight=13.053496, address='home', name='gj5vYSKI'}
    info: CarePet.Sensor[0]
        sensor = Sensor{petId=6db45498-244c-460c-a567-2098ab7fb848, sensorId=3c6ba0fc-9516-4dc4-8ab5-1ebe50f37f03, type='T'}
    info: CarePet.Sensor[0]
        sensor = Sensor{petId=6db45498-244c-460c-a567-2098ab7fb848, sensorId=33d51816-5df6-4e62-ad0b-4e039b198dab, type='P'}
    info: CarePet.Sensor[0]
        sensor = Sensor{petId=6db45498-244c-460c-a567-2098ab7fb848, sensorId=d03a3de6-5f19-4cdc-854c-392f3212b70c, type='L'}
    info: CarePet.Sensor[0]
        sensor = Sensor{petId=6db45498-244c-460c-a567-2098ab7fb848, sensorId=35cbd6db-a580-44bb-ac14-e82f8c1fb727, type='R'}
    info: CarePet.Sensor[0]
        Measure{sensorId=3c6ba0fc-9516-4dc4-8ab5-1ebe50f37f03, ts=8/25/2025 9:16:02 AM +00:00, value=102}
    info: CarePet.Sensor[0]
        Measure{sensorId=33d51816-5df6-4e62-ad0b-4e039b198dab, ts=8/25/2025 9:16:02 AM +00:00, value=89}
    info: CarePet.Sensor[0]
        Measure{sensorId=d03a3de6-5f19-4cdc-854c-392f3212b70c, ts=8/25/2025 9:16:02 AM +00:00, value=0.30064157}
    info: CarePet.Sensor[0]
        Measure{sensorId=35cbd6db-a580-44bb-ac14-e82f8c1fb727, ts=8/25/2025 9:16:02 AM +00:00, value=36}
    info: CarePet.Sensor[0]
        pushing data
    info: CarePet.Sensor[0]
        Measure{sensorId=3c6ba0fc-9516-4dc4-8ab5-1ebe50f37f03, ts=8/25/2025 9:17:02 AM +00:00, value=105}
    info: CarePet.Sensor[0]
        Measure{sensorId=33d51816-5df6-4e62-ad0b-4e039b198dab, ts=8/25/2025 9:17:02 AM +00:00, value=93}
    info: CarePet.Sensor[0]
        Measure{sensorId=d03a3de6-5f19-4cdc-854c-392f3212b70c, ts=8/25/2025 9:17:02 AM +00:00, value=2.0097651}
    info: CarePet.Sensor[0]
        Measure{sensorId=35cbd6db-a580-44bb-ac14-e82f8c1fb727, ts=8/25/2025 9:17:02 AM +00:00, value=37}
    info: CarePet.Sensor[0]
        pushing data
      ...

You can stop the data generation with `Ctrl+C`.

To start the web server:

    $ dotnet run --project CarePet.csproj --hosts $NODE1 --datacenter datacenter1

Expected output:

    info: CarePet.Program[0]
          Starting CarePet server on port 8000...
    info: Microsoft.Hosting.Lifetime[14]
          Now listening on: http://0.0.0.0:8000
    info: Microsoft.Hosting.Lifetime[0]
          Application started. Press Ctrl+C to shut down.

Now you can open the Swagger UI at http://localhost:8000/swagger or test the endpoints directly.

Endpoints
---

Use available pets/owners/sensors. To populate them use `sensor` command.

Get owner information:

    $ curl http://127.0.0.1:8000/api/owner/{owner-id}

Expected output:

    {"ownerId":"f1c8ca4c-06c5-4b1a-9e39-8e3d4d5f6789","address":"123 Main St, New York, NY","name":"John Doe"}

Get pets by owner:

    $ curl http://127.0.0.1:8000/api/pet/owner/{owner-id}

Expected output:

    [{"ownerId":"f1c8ca4c-06c5-4b1a-9e39-8e3d4d5f6789","petId":"a1b2c3d4-e5f6-7890-1234-567890abcdef","chipId":"CHIP123456","species":"Dog","breed":"Labrador","color":"Golden","gender":"Male","age":3,"weight":25.5,"address":"","name":"Buddy"}]

Get sensors for a pet:

    $ curl http://127.0.0.1:8000/api/sensor/pet/{pet-id}

Expected output:

    [{"petId":"a1b2c3d4-e5f6-7890-1234-567890abcdef","sensorId":"5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d","type":"T"},{"petId":"a1b2c3d4-e5f6-7890-1234-567890abcdef","sensorId":"e81915d6-1155-45e4-9174-c58e4cb8cecf","type":"P"}]

Get measurements for a sensor with time range:

    $ curl "http://127.0.0.1:8000/api/sensor/{sensor-id}/measurements?from=2025-08-06T00:00:00Z&to=2025-08-07T00:00:00Z"

Expected output:

    [{"sensorId":"5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d","timestamp":"2025-08-06T12:30:00Z","value":38.5}]

Get daily averaged values:

    $ curl http://127.0.0.1:8000/api/sensor/{sensor-id}/values/day/2025-08-06

Expected output:

    [0,0,0,0,0,0,0,0,0,0,0,0,38.5,39.1,0,0,0,0,0,0,0,0,0,0]

Structure
---

Package structure is as follows:

| Name                        | Purpose                                   |
| --------------------------- | ----------------------------------------- |
| **Commands/**               | CLI command implementations               |
| **Commands/MigrateCommand** | Database schema installation              |
| **Commands/SensorCommand**  | Pet collar simulation                     |
| **Controllers/**            | REST API controllers                      |
| **Database/**               | ScyllaDB connection and repositories      |
| **Models/**                 | Data models (Owner, Pet, Sensor, etc.)   |
| **Config/**                 | Configuration classes                     |
| **cql/**                    | Database schema files                     |

API
---

The application uses ASP.NET Core to serve REST endpoints with Swagger/OpenAPI documentation available at `/swagger`.

The API provides the following endpoints:

- `GET /api/owner/{id}` - Get owner information
- `GET /api/pet/owner/{ownerId}` - Get all pets for an owner
- `GET /api/pet/{ownerId}/{petId}` - Get specific pet information
- `GET /api/sensor/pet/{petId}` - Get all sensors for a pet
- `GET /api/sensor/{sensorId}/measurements` - Get measurements with optional time range
- `GET /api/sensor/{sensorId}/values/day/{date}` - Get hourly averaged values for a day

Implementation
---

Collars are small devices that attach to pets and collect data
with the help of different sensors. After the data is collected
it may be delivered to the central database for the analysis and
health status checking.

Collar code sits in the `Commands/SensorCommand` and uses the ScyllaDB C# driver
to connect to the database directly and publish its data.

Collar gathers sensors measurements, aggregates data in a buffer and
sends it every minute or when the buffer is full.

Overall all applications in this repository use the ScyllaDB C# driver for:

* Relational Object Mapping (ORM)
* Migration support (DDL)
* Connection session management (Session, Cluster)
* Query builder

The application leverages .NET's built-in dependency injection, configuration management,
and logging frameworks for a robust and maintainable codebase.

### Data Model

The application uses the following sensor types:

- **Temperature** (T): Pet body temperature (36-46°C)
- **Pulse** (P): Heart rate (60-110 bpm)
- **Location** (L): GPS coordinates
- **Respiration** (R): Breathing rate (10-30 breaths/min)

### Features

- **ScyllaDB C# Driver**: Optimized for ScyllaDB with connection pooling and prepared statements
- **Modern C# patterns**: Async/await, nullable reference types, dependency injection
- **Command-line interface**: Built with CommandLineParser for easy CLI usage
- **Configuration**: Supports appsettings.json, environment variables, and command-line options
- **API documentation**: Automatic Swagger/OpenAPI documentation generation
- **Docker support**: Complete containerization with Docker and Docker Compose

### Configuration

The application can be configured through:

1. `appsettings.json` file
2. Environment variables (with `Database__` prefix)
3. Command-line arguments

Example configuration:

```json
{
  "Database": {
    "Keyspace": "carepet",
    "Hosts": ["localhost"],
    "Port": 9042,
    "Datacenter": "datacenter1"
  }
}
```

### Commands

All commands support the following ScyllaDB connection options:

- `--hosts`: ScyllaDB host addresses (default: localhost)
- `--port`: ScyllaDB port (default: 9042)
- `--datacenter`: ScyllaDB datacenter name (default: datacenter1)
- `--keyspace`: Keyspace name (default: carepet)

Additional command-specific options:

**Migrate command:**
- `--replication-factor`: Keyspace replication factor (default: 3)

**Sensor command:**
- `--interval`: Measurement interval in milliseconds (default: 1000)
- `--buffer-size`: Buffer size for batching measurements (default: 100)

**Server command:**
- `--port`: HTTP server port (default: 8000)

Use `--help` with any command to see all available options:

    $ dotnet run --project CarePet.Migrate.csproj --help
    $ dotnet run --project CarePet.Sensor.csproj --help
    $ dotnet run --project CarePet.csproj --help
