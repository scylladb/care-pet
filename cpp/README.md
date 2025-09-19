Care Pet ScyllaDB IoT example (C++)
===

This example project demonstrates a generic IoT use case
for ScyllaDB in C++.
The documentation for this application and guided exercise is [here](../docs/build-with-cpp.md).

The application allows tracking of pets health indicators
and consist of three parts, all within a single executable:

- `migrate` - creates the `carepet` keyspace and tables
- `sensor` - generates a pet health data and pushes it into the storage
- `server` - REST API service for tracking pets health state

Quick Start
---

Prerequisites:

- A C++20 compatible compiler (e.g., GCC 10+, Clang 12+)
- [CMake](https://cmake.org/install/) (version 3.30 or later)
- [Boost libraries](https://www.boost.org/) (version 1.83.0 or later, specifically program_options, system, filesystem, json, url)
- [Scylla C++ Driver](https://github.com/scylladb/cpp-rs-driver)
- [docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

To run a local ScyllaDB cluster consisting of three nodes with
the help of `docker` and `docker-compose` execute from the `cpp` directory:

    $ docker-compose up -d

Docker-compose will spin up three nodes: `carepet-scylla1`, `carepet-scylla2`
and `carepet-scylla3`. You can access them with the `docker` command.

To execute CQLSH:

    $ docker exec -it carepet-scylla1 cqlsh

To build the C++ application, run the following from the `cpp` directory:

    $ mkdir -p build
    $ cd build
    $ cmake ..
    $ make

This will create the `care-pet` executable in the `cpp/build` directory.

To initialize the database execute:

    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./build/care-pet migrate --scylla-host $NODE1

You can check the database structure with:

    $ docker exec -it carepet-scylla1 cqlsh
    cqlsh> DESCRIBE carepet
    cqlsh> exit

To start pet collar simulation execute the following in a separate terminal:

    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./build/care-pet sensor --scylla-host $NODE1 --seconds 300

It should print IDs of created Owner, Pet, and Sensors. Save them - you'll use them in a moment to query the data.

To start the REST API service execute the following in a separate terminal:

    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./build/care-pet server --scylla-host $NODE1 --host 0.0.0.0 --port 8080

Now you can send HTTP requests to `http://127.0.0.1:8080/`, for example from the CLI.

To read an owner's data you can use a saved `owner_id` as follows:

    $ curl http://127.0.0.1:8080/owner/{owner_id}

To list the owner's pets use:

    $ curl http://127.0.0.1:8080/owner/{owner_id}/pets

To list a pet's sensors use:

    $ curl http://127.0.0.1:8080/pet/{pet_id}/sensors

To review the pet's sensors data use:

    $ curl http://127.0.0.1:8080/sensor/{sensor_id}/values?from=...&to=...

`from` and `to` should be timestamps formatted like `2025-09-20T13:43:25Z`.

To read the pet's daily average per sensor use:

    $ curl http://127.0.0.1:8080/api/sensor/{sensor_id}/values/day/{date}

`date` parameter should be formatted like `2025-09-30`.

Structure
---

The C++ project is structured as follows:

| Name              | Purpose                                     |
| ----              | -------                                     |
| /src/main.cpp     | Main entry point, handles command line args |
| /src/common       | Shared code (database, models)              |
| /src/migrate      | Database schema migration logic             |
| /src/sensor       | Pet collar simulation logic                 |
| /src/server       | Web application backend (REST API)          |
| /data             | CQL schema files                            |
| CMakeLists.txt    | Main CMake build file                       |

Implementation
---

The application uses the [Scylla C++ Driver](https://github.com/scylladb/cpp-rs-driver) to interact with the database.
The REST API server is built using [Boost.Beast](https://www.boost.org/doc/libs/release/libs/beast/).

The `main.cpp` file uses `Boost.ProgramOptions` to parse command-line arguments and determine which mode to run (`migrate`, `sensor`, or `server`).

The database logic is encapsulated in the `Database` class in `src/common/database.hpp` and `src/common/database.cpp`.

The web server in `src/server` handles HTTP requests and translates them into database queries. The handlers in `src/server/handlers.cpp` contain the logic for each API endpoint.
