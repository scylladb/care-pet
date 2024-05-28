# Care Pet ScyllaDB IoT example

This example project demonstrates a generic IoT use case for ScyllaDB in Rust.

The documentation for this application and guided exercise is [here](../docs).

The application allows tracking of pets health indicators and consist of three parts:

- migrate (`/database/migrate/mod.rs`) - creates the `carepet` keyspace and tables
- sensor (`/stressers/sensor/mod.rs`) - generates a pet health data and pushes it into the storage
- stress (`/stressers/stress/mod.rs`) - generates  a lot of data for the stress testing
- web app (`/http/mod.rs`) - REST API service for tracking pets health state

## Quick Start

Prerequisites:

- [Rust](https://rustup.rs/) at least 1.57
- [docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

To run a local ScyllaDB cluster consisting of three nodes with
the help of `docker` and `docker-compose` execute:

    $ docker-compose up -d

Docker-compose will spin up three nodes: `carepet-scylla1`, `carepet-scylla2`
and `carepet-scylla3`. You can access them with the `docker` command.

To execute CQLSH:

    $ docker exec -it carepet-scylla1 cqlsh

To execute nodetool:

    $ docker exec -it carepet-scylla1 nodetool status

Shell:

    $ docker exec -it carepet-scylla1 shell

The port of `carepet-scylla1` is published to the host on port `9042` using the host IP address. 
To get the host IP address run:

    $ HOST_IP=$(hostname -I | awk '{print $1}')

To initialize database execute:

    $ cargo run --bin migrate -- --hosts $HOST_IP

Expected output:

    [2021-12-24T00:39:00Z INFO  migrate] Bootstrapping database...
    [2021-12-24T00:39:00Z INFO  care_pet::db] Connecting to 172.nnn.nnn.nnn
    [2021-12-24T00:39:00Z INFO  care_pet::db] Creating keyspace carepet
    [2021-12-24T00:39:00Z INFO  care_pet::db] Keyspace carepet created
    [2021-12-24T00:39:00Z INFO  care_pet::db] Migrating database
    [2021-12-24T00:39:00Z INFO  care_pet::db] Executed migration script 1/5
    [2021-12-24T00:39:00Z INFO  care_pet::db] Executed migration script 2/5
    [2021-12-24T00:39:00Z INFO  care_pet::db] Executed migration script 3/5
    [2021-12-24T00:39:00Z INFO  care_pet::db] Executed migration script 4/5
    [2021-12-24T00:39:00Z INFO  care_pet::db] Executed migration script 5/5
    [2021-12-24T00:39:00Z INFO  care_pet::db] Database migrated

You can check the database structure with:

    $ docker exec -it carepet-scylla1 cqlsh
    cqlsh> DESCRIBE KEYSPACES

    carepet        system_auth  system_distributed_everywhere  system_traces
    system_schema  system       system_distributed

    cqlsh> USE carepet;
    cqlsh:carepet> DESCRIBE TABLES

    owner  pet  sensor  sensor_avg  measurement

    cqlsh:carepet> DESCRIBE TABLE pet

    CREATE TABLE carepet.pet (
        owner_id uuid,
        pet_id uuid,
        address text,
        age int,
        breed text,
        chip_id text,
        color text,
        gender text,
        name text,
        species text,
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


    cqlsh:carepet> exit

To start pet collar simulation execute the following in the separate terminal:

    $ HOST_IP=$(hostname -I | awk '{print $1}')
    $ cargo run --bin sensor -- --hosts $HOST_IP --measure 5s --buffer-interval 1m

Expected output:

    [2021-12-24T00:39:56Z INFO  sensor] Welcome to the Pet collar simulator
    [2021-12-24T00:39:56Z INFO  care_pet::db] Connecting to 172.nnn.nnn.nnn
    [2021-12-24T00:39:56Z INFO  sensor] New owner # 26b5a174-57f4-4bd8-928a-d5ed065b211b
    [2021-12-24T00:39:56Z INFO  sensor] New pet # 3e0a8390-b27f-4f8b-816e-904dcf2bf40c
    [2021-12-24T00:40:01Z INFO  sensor] sensor # 50ed149f-d657-478a-9e15-aedf43319e25 type R new measure 37.6081 ts 2021-12-24T00:40:01.535000000Z
    [2021-12-24T00:40:06Z INFO  sensor] sensor # 50ed149f-d657-478a-9e15-aedf43319e25 type R new measure 35.536953 ts 2021-12-24T00:40:06.537000000Z
    [2021-12-24T00:40:11Z INFO  sensor] sensor # 50ed149f-d657-478a-9e15-aedf43319e25 type R new measure 34.207542 ts 2021-12-24T00:40:11.539000000Z
    ...

In a minute (a `--buffer-interval`) you will see a data push (`Pushing data`) log line.
That means that the collar has been pushed buffered measurements to the app.

Write down the pet Owner ID (ID is something after the `#` sign without trailing spaces).
To start REST API service execute the following in the separate terminal:

    $ cargo run server 

Expected output:

    2021-12-24T00:32:48Z INFO  care_pet::db] Connecting to 172.nnn.nnn.nnn
    [2021-12-24T00:32:48Z WARN  rocket::config::config] 🔧 Configured for debug.
    [2021-12-24T00:32:48Z WARN  rocket::config::config] address: 0.0.0.0
    [2021-12-24T00:32:48Z WARN  rocket::config::config] port: 8000
    [2021-12-24T00:32:48Z WARN  rocket::config::config] workers: 16
    [2021-12-24T00:32:48Z WARN  rocket::config::config] ident: Rocket
    [2021-12-24T00:32:48Z WARN  rocket::config::config] keep-alive: 5s
    ...

Now you can open `http://0.0.0.0:8000/` in the browser or send an HTTP request from the CLI:

    $ curl -v http://0.0.0.0:8000/

    > GET / HTTP/1.1
    > Host: 0.0.0.0:8000
    > User-Agent: curl/7.71.1
    > Accept: */*
    >
    * Mark bundle as not supporting multiuse
    < HTTP/1.1 404 Not Found
    < Content-Type: application/json
    < Date: Thu, 06 Aug 2020 14:47:41 GMT
    < Content-Length: 45
    < Connection: close
    <
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8">
        <title>404 Not Found</title>
    </head>
    <body align="center">
        <div role="main" align="center">
            <h1>404: Not Found</h1>
            <p>The requested resource could not be found.</p>
            <hr />
        </div>
        <div role="contentinfo" align="center">
            <small>Rocket</small>
        </div>
    </body>
    * Connection #0 to host 0.0.0.0 left intact
    </html>⏎

This is ok. If you see this page in the end with 404, it means everything works as expected.
To read an owner data you can use saved `owner_id` as follows:

    $ curl http://0.0.0.0:8000/api/owner/{owner_id}

For example:

    $ curl http://0.0.0.0:8000/api/owner/a05fd0df-0f97-4eec-a211-cad28a6e5360

Expected result:

    {"address":"home","name":"gmwjgsap","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360"}

To list the owners pets use:

    $ curl http://0.0.0.0:8000/api/owner/{owner_id}/pets

For example:

    $ curl http://0.0.0.0:8000/api/owner/a05fd0df-0f97-4eec-a211-cad28a6e5360/pets

Expected output:

    [{"address":"home","age":57,"name":"tlmodylu","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360","pet_id":"a52adc4e-7cf4-47ca-b561-3ceec9382917","weight":5}]

To list pet's sensors use:

    $ curl http://0.0.0.0:8000/api/pet/{pet_id}/sensors

For example:

    $ curl http://0.0.0.0:8000/api/pet/cef72f58-fc78-4cae-92ae-fb3c3eed35c4/sensors

    [{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d","type":"L"},{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"5c70cd8a-d9a6-416f-afd6-c99f90578d99","type":"R"},{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"fbefa67a-ceb1-4dcc-bbf1-c90d71176857","type":"L"}]

To review the pet's sensors data use:

    $ curl http://0.0.0.0:8000/api/sensor/{sensor_id}/values?from=2006-01-02T15:04:05Z07:00&to=2006-01-02T15:04:05Z07:00

For example:

    $  curl http://0.0.0.0:8000/api/sensor/5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d/values\?from\="2020-08-06T00:00:00Z"\&to\="2020-08-06T23:59:59Z"

Expected output:

    [51.360596,26.737432,77.88015,...]

To read the pet's daily average per sensor use:

    $ curl http://0.0.0.0:8000/api/sensor/{sensor_id}/values/day/{date}

For example:

    $ curl http://0.0.0.0:8000/api/sensor/5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d/values/day/2020-08-06

Expected output:

    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,42.55736]

## Structure

Package structure is as follows:

| Name              | Purpose                                         |
|-------------------|-------------------------------------------------|
| /                 | base application and common application code    |
| /database         | default scylla connection                       |
| /database/migrate | install database schema                         |
| /http             | web application, controllers and error handling |
| /db               | database specific utilities                     |
| /stressers        | stressing profiles: sensor, loadtest(stress)    |
| /model            | application models                              |
| /repositories     | database application layer                      |

## Implementation

Collars are small devices that attach to pets and collect data
with the help of different sensors. After the data is collected
it may be delivered to the central database for the analysis and
health status checking.

Collar code sits in the `/stressers/sensor` and uses [Scylla Rust Driver](https://github.com/scylladb/scylla-rust-driver) 
to connect to the database directly and publish its data.
Collar gathers sensors measurements, aggregates data in a buffer and
sends it every hour.

Overall all applications in this repository use [Scylla Rust Driver](https://github.com/scylladb/scylla-rust-driver) for:

- Relational Object Mapping (ORM)
- Build Queries
- Migrate database schemas

The web application REST API server resides in `/http/mod.rs` and uses
[rocket.rs](https://rocket.rs/). API handlers reside in `/http/controllers`.
Most of the queries are reads.

The application is capable of caching sensor measurements data
on hourly basis. It uses lazy evaluation to manage `sensor_avg`.
It can be viewed as an application-level lazy-evaluated
materialized view.

The algorithm is simple and resides in `/http/controllers/sensors_controller.rs`:

- read `sensor_avg`
- if no data, read `measurement` data, aggregate in memory, save
- serve request

## Architecture

    Pet --> Sensor --> ScyllaDB <-> REST API Server <-> User

## How to start a new project with Rust

Install Rust. Create a repository. Clone it. Execute inside of
your repository:

    $ cargo new project_name

Now in `project_name/Cargo.toml`, under `dependencies` specify:

    [dependencies]
    scylla = "0.13"
    tokio = {version = "1.1", features = ["full"]}

Now you are ready to connect to the database and start working.
To connect to the database, do the following:

```rust
use scylla::{Session, SessionBuilder, IntoTypedRows};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = "0.0.0.0:9042";

    let session: Session = SessionBuilder::new().known_node(uri).build().await?;
    // ...
    Ok(())
}
```

Now you can issue CQL commands:

```rust
if let Some(rows) = session.query("SELECT a, b, c FROM ks.t", &[]).await?.rows {
    for row in rows.into_typed::<(i32, i32, String)>() {
        let (a, b, c) = row?;
        println!("a, b, c: {}, {}, {}", a, b, c);
    }
}
```

Or save models:

```rust
let to_insert: i32 = 12345;
session
    .query("INSERT INTO keyspace.table (a) VALUES(?)", (to_insert,))
    .await?;
```

For more details, check out `/handler`, `/db` and `/model` packages.

## Links

- https://hub.docker.com/r/scylladb/scylla/
- https://github.com/scylladb/scylla-rust-driver
