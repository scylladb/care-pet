# Care Pet ScyllaDB IoT example

This is an example project that demonstrates a generic IoT use case with
ScyllaDB in Python.

The project simulates an IoT application for pet owners to monitor a variety
of metrics about their pets (for example heart rate or temperature).

The application has three modules:

* Migrate (`python src/migrate.py`) - creates keyspace and tables in ScyllaDB
* Sensor (`python src/sensor.py`) - generates random IoT data and inserts it into ScyllaDB
* API (`python src/api.py`) - REST API service to fetch data from ScyllaDB

## Get started

### Prerequisites:
* [Python 3.7+](https://www.python.org/downloads/)
* [Virtualenv](https://virtualenv.pypa.io/en/latest/installation.html)
* [docker](https://www.docker.com/)
* [docker-compose](https://docs.docker.com/compose/)

### Clone repository and install dependencies
Clone the repository and open the root directory of the project:
```bash
git clone https://github.com/zseta/carepet-python
cd carepet-python
```

Create a new virtual environment and activate it:
```bash
virtualenv env
source env/bin/activate
```

Install all Python dependencies:
```bash
pip install -r requirements.txt
```

### Start Docker containers (skip this if you use Scylla Cloud)
Spin up a local ScyllaDB cluster with three nodes using `docker` and `docker-compose`:
```bash
docker-compose up -d

Creating carepet-scylla3 ... done
Creating carepet-scylla2 ... done
Creating carepet-scylla1 ... done
```

This command starts three ScyllaDB nodes in containers:
* `carepet-scylla1`
* `carepet-scylla2`
* `carepet-scylla3`

You can inspect any of these nodes by using the `docker inspect` command,
for example:
```bash
docker inspect carepet-scylla1

[
    {
        "Id": "c87128b7d0ca4a31a84da78875c8b4181283c34783b6b0a78bffbacbbe45fcc2",
        "Created": "2023-01-08T21:17:13.212585687Z",
        "Path": "/docker-entrypoint.py",
        "Args": [
            "--smp",
            "1"
        ],
        "State": {
            "Status": "running",
            "Running": true,
...
```

### Connect to ScyllaDB and create the database schema
To connect to your ScyllaDB storage within the container, you need to know the
IP address of one of the running nodes.
This is how you can get the IP address of the first node running in the container:
```bash
docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1
```

You will need to reference this value multiple times later so if it's easier
for you can save it as a variable `NODE1`:
```bash
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
```

Now you can run the migration script that creates the required keyspace and tables:
```bash
python src/migrate.py -h $NODE1

Creating keyspace...
Done.
Migrating database...
Done.
```

See the database schema using [cqlsh](https://cassandra.apache.org/doc/latest/cassandra/tools/cqlsh.html) in the container:

```bash
docker exec -it carepet-scylla1 cqlsh
cqlsh> DESCRIBE KEYSPACES;

carepet        system_auth  system_distributed_everywhere  system_traces
system_schema  system       system_distributed 

cqlsh> USE carepet;
cqlsh:carepet> DESCRIBE TABLES;

owner  pet  sensor  sensor_avg  measurement

cqlsh:carepet> DESCRIBE TABLE pet;

CREATE TABLE carepet.pet (
    owner_id uuid,
    pet_id uuid,
    address text,
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
    AND dclocal_read_repair_chance = 0.0
    AND default_time_to_live = 0
    AND gc_grace_seconds = 864000
    AND max_index_interval = 2048
    AND memtable_flush_period_in_ms = 0
    AND min_index_interval = 128
    AND read_repair_chance = 0.0
    AND speculative_retry = '99.0PERCENTILE';

cqlsh:carepet> exit;


```

At this point you have ScyllaDB running with the correct keyspace and tables.

### Generate and ingest IoT data
Start ingesting IoT data (it's suggested to do this in a new separate terminal
because this process runs indefinitely). Make sure you're still in the virtual
environment:
```bash
source env/bin/activate
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
python src/sensor.py -h $NODE1 --measure 2 --buffer-interval 10

Welcome to the Pet collar simulator
New owner # 1cfbc0e5-6b05-476d-b170-2660cf40c02a
New pet # 1a0800ee-7643-4794-af7b-2ecaaf7078fc
New sensor(0) # b6155934-bd4e-47de-8649-1fad447aa036
New sensor(1) # d2c62c4d-9621-469d-b62c-41ef2271fca7
sensor # b6155934-bd4e-47de-8649-1fad447aa036 type T, new measure: 100.55118431400851, ts: 2023-01-08 17:36:17.126374
sensor # d2c62c4d-9621-469d-b62c-41ef2271fca7 type L, new measure: 37.486651732296835, ts: 2023-01-08 17:36:17.126516
```

This command starts a script that generates and ingests random IoT data coming
from two sensors every other second and inserts the data in batches
every ten seconds. Whenever you see `Pushing data` in the command line that is
when data actually gets insterted into ScyllaDB.

Optional: You can modify the frequency of the generated data by changing the
`--measure` and `--buffer-interval` arguments. For example,
you can generate new data points every three seconds and insert the batches
every 30 seconds:
```bash
source env/bin/activate
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
python src/sensor.py -h $NODE1 --measure 3 --buffer-interval 30
```

You can run multiple ingestion processes in parallel if you wish.

### Set up and test REST API
In a new terminal, start running the API server (make sure that `port 8000` is free):
```bash
source env/bin/activate
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
python src/api.py -h $NODE1

INFO:     Started server process [696274]
INFO:     Waiting for application startup.
INFO:     Application startup complete.
INFO:     Uvicorn running on http://127.0.0.1:8000 (Press CTRL+C to quit)
```

The API server is running on `http://127.0.0.1:8000`. Test with your
browser, or curl, if it works properly:
```bash
curl http://127.0.0.1:8000

{"message":"Pet collar simulator API"}
```

Next, you will test the following API endpoints:
* `/api/owner/{owner_id}`

    Returns all available data fields about the owner.
* `/api/owner/{owner_id}/pets`

    Returns the owner's pets.
* `/api/pet/{pet_id}/sensors`

    Returns all the sensors of a pet.

To test these endpoints, you need to provide either an `owner_id` or a `pet_id`
as URL path parameter. You can get these values by copying them from the
beginning of output of the ingestion script:
```bash
source env/bin/activate
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
python src/sensor.py -h $NODE1 --measure 1 --buffer-interval 6

Welcome to the Pet collar simulator
New owner # 1cfbc0e5-6b05-476d-b170-2660cf40c02a <-- This is what you need!
New pet # 1a0800ee-7643-4794-af7b-2ecaaf7078fc <-- This is what you need!
New sensor(0) # b6155934-bd4e-47de-8649-1fad447aa036
New sensor(1) # d2c62c4d-9621-469d-b62c-41ef2271fca7
```

Copy the UUID values right after "New owner #" and "New pet #". A UUID value
looks like this:
```
1cfbc0e5-6b05-476d-b170-2660cf40c02a
```

**Test `/api/owner/{owner_id}`**

Paste the owner id from the terminal into the endpoint URL and open it with
your browser or use `curl`, for example:
```bash
curl http://127.0.0.1:8000/api/owner/4f42fb80-c209-4d19-8c43-daf554f1be23

{"owner_id":"4f42fb80-c209-4d19-8c43-daf554f1be23","address":"home","name":"Vito Russell"}
```

**Test `/api/owner/{owner_id}/pets`**

Use the same owner id value to test this endpoint, for example:
```bash
curl http://127.0.0.1:8000/api/owner/4f42fb80-c209-4d19-8c43-daf554f1be23/pets

[{"owner_id":"4f42fb80-c209-4d19-8c43-daf554f1be23","pet_id":"44f1624e-07c2-4971-85a5-85b9ad1ff142","address":"home","age":20,"name":"Duke","weight":14.41481876373291}]
```

**Test `/api/pet/{pet_id}/sensors`**

Finally, use a pet id to test this endpoint, for example:
```bash
curl http://127.0.0.1:8000/api/pet/44f1624e-07c2-4971-85a5-85b9ad1ff142/sensors

[{"pet_id":"44f1624e-07c2-4971-85a5-85b9ad1ff142","sensor_id":"4bb1d214-712b-453b-b53a-ac5d4df4a1f8","type":"T"},{"pet_id":"44f1624e-07c2-4971-85a5-85b9ad1ff142","sensor_id":"e81915d6-1155-45e4-9174-c58e4cb8cecf","type":"L"}]
```

## Structure
Package structure:

| Name                                    | Purpose                              |
| ----------------------------------------| -------------------------------------|
| [/src/db](/src/db)                      | Database config and client folder    |
| [/src/db/cql](/src/db/cql)              | CQL scripts                          |
| [/src/db/client](/src/db/client.py)      | ScyllaDB client library              |
| [/src/server](/src/server)              | FastAPI application folder           |
| [/src/server/app.py](/src/server/app.py)| FastAPI application                  |
| [/src/api.py](/src/api.py )             | Script to start the API server       |
| [/src/migrate.py](/src/migrate.py)      | Schema creation                      |
| [/src/sensor.py](/src/sensor.py)        | IoT data ingestion                   |
