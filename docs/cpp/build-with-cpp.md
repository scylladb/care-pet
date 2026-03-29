# Build an IoT App with C++

## Architecture

In this section, we will walk you through the CarePet commands and explain the code behind them.
The project is a single executable `care-pet` that can be run in three different modes:

-   `migrate` - Creates the `carepet` keyspace and tables.
-   `sensor` - Generates pet health data and pushes it into the storage.
-   `server` - REST API service for tracking pets' health state.

The application logic is split into corresponding components: `migrate`, `sensor`, and `server`. There is also a `common` component that contains shared code, such as database connection logic and data models.

## Building the project

The project uses CMake for building. To build the project, you need to have a C++ compiler (like GCC or Clang), CMake, and the Boost library installed.

From the `cpp` directory, run the following commands:

```bash
mkdir -p build
cd build
cmake ..
make
```

This will create the `care-pet` executable in the `cpp/build` directory.

## Migrate

The `./build/care-pet migrate --scylla-host $NODE1` command executes the migration logic. The main function in `src/main.cpp` parses the command-line arguments and, for the `migrate` mode, calls the `run_migrate` function from `src/migrate/migrate.cpp`.

The `run_migrate` function connects to the ScyllaDB cluster and executes the CQL commands from the `data/care-pet-ddl.cql` file to create the necessary keyspace and tables.

```cpp
// In src/migrate/migrate.cpp
void run_migrate(const po::variables_map& vm) {
    std::cout << "Running in migrate mode\n";
    Database db(vm["scylla-host"].as<std::string>());
    db.connect();

    auto ddl_files = vm["ddl-file"].as<std::vector<std::string>>();
    for (const auto& file_path : ddl_files) {
        std::ifstream file(file_path);
        if (!file.is_open()) {
            std::cerr << "Error: Could not open DDL file '" << file_path << "'\n";
            continue;
        }
        std::string ddl_query((std::istreambuf_iterator<char>(file)),
                              std::istreambuf_iterator<char>());
        db.execute_query(ddl_query);
        std::cout << "Executed DDL from " << file_path << "\n";
    }
}
```

The `care-pet-ddl.cql` file contains `CREATE KEYSPACE` and `CREATE TABLE` statements for `owner`, `pet`, `sensor`, `measurement`, and `sensor_avg` tables, similar to the Java example.

You can check the database structure with:

```bash
$ docker exec -it carepet-scylla1 cqlsh
cqlsh> USE carepet;
cqlsh:carepet> DESCRIBE TABLES
cqlsh:carepet> DESCRIBE TABLE pet
```

## Sensor

The sensor service simulates the collar's activity. You can use the following command to run the sensor service:

```bash
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' care-pet-scylla1)
$ ./build/care-pet sensor --scylla-host $NODE1 --seconds 60
```

This command executes the `run_sensor` function from `src/sensor/sensor.cpp`. This function simulates a pet collar, generating random data for an owner, a pet, and its sensors. It then periodically sends measurement data to the database.

```cpp
// In src/sensor/sensor.cpp
void run_sensor(const po::variables_map& vm) {
    std::cout << "Running in sensor mode\n";
    Database db(vm["scylla-host"].as<std::string>());
    db.connect("carepet");

    // ... create random owner, pet, and sensors ...
    // ... save them to the database ...

    int seconds = vm["seconds"].as<int>();
    auto start_time = std::chrono::steady_clock::now();

    while (std::chrono::steady_clock::now() - start_time < std::chrono::seconds(seconds)) {
        for (const auto& s : sensors) {
            Measurement m = read_sensor_data(s);
            // ... insert measurement into the database ...
        }
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }
}
```

The code uses prepared statements to insert data into the `measurement` table efficiently.

## Server

The server service is a REST API for tracking the pets’ health state. The service allows you to query the database via HTTP.

Run the following commands to start the server:

```bash
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' care-pet-scylla1)
$ ./build/care-pet server --scylla-host $NODE1 --host 0.0.0.0 --port 8080
```

This starts an HTTP server using Boost.Beast. The server exposes several endpoints to retrieve data from the database. The handlers for these endpoints are defined in `src/server/handlers.cpp`.

For example, to get an owner's data, you can use:

`$ curl http://127.0.0.1:8080/api/owner/{id}`

The server also aggregates the data and saves it to the database in the `sensor_avg` table, similar to the Java implementation.

## Resources

* [ScyllaDB C++ Driver on Github](https://github.com/scylladb/scylla-cpp-driver)
