# Care Pet ScyllaDB IoT example

This example project demonstrates a generic IoT use case for ScyllaDB in PHP.

Here you will find a list with possible drivers to integrate.

| PHP Version | Driver                                                                         |
|-------------|--------------------------------------------------------------------------------|
| PHP 7.1 [x] | [DataStax PHP Driver](https://github.com/datastax/php-driver)                  |
| PHP 8.1     | [Cassandra PHP Driver (dev)](https://github.com/qkdreyer/cassandra-php-driver) |

The documentation for this application and guided exercise is [here](../docs).

## Quick Start
The application allows tracking of pets health indicators and consist in a CLI of three parts:

| Command             | Description                                                |
|---------------------|------------------------------------------------------------|
| php scylla migrate  | creates the `carepet` keyspace and tables                  |
| php scylla simulate | generates a pet health data and pushes it into the storage |
| php scylla serve    | REST API service for tracking pets health state            |


Prerequisites:
- [docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

## Setup

To run a local **ScyllaDB cluster** consisting of three nodes and the **PHP Workspace** with
the help of `docker` and `docker-compose` execute:

````shell
$ docker-compose up -d
````    

Docker-compose will spin up three nodes which is:

- carepet-scylla1
- carepet-scylla2
- carepet-scylla3

If you want to see your containers running, run the `docker ps` command, and you should see something like this:

`````shell
$ docker ps
CONTAINER ID   IMAGE                    COMMAND                  CREATED       STATUS       PORTS                                                                      NAMES
14a656685517   care-pet-php-workspace   "/bin/sh -c /bin/bas…"   1 minute ago   Up 1 minute   9000/tcp                                                                   workspace-php
4e351dfe3987   scylladb/scylla          "/docker-entrypoint.…"   1 minute ago   Up 1 minute   22/tcp, 7000-7001/tcp, 7199/tcp, 9042/tcp, 9160/tcp, 9180/tcp, 10000/tcp   carepet-scylla2
9e7e4d3992df   scylladb/scylla          "/docker-entrypoint.…"   1 minute ago   Up 1 minute   22/tcp, 7000-7001/tcp, 7199/tcp, 9042/tcp, 9160/tcp, 9180/tcp, 10000/tcp   carepet-scylla3
7e2b1b94389b   scylladb/scylla          "/docker-entrypoint.…"   1 minute ago   Up 1 minute   22/tcp, 7000-7001/tcp, 7199/tcp, 9042/tcp, 9160/tcp, 9180/tcp, 10000/tcp   carepet-scylla1
`````

> If you have any error regarding "premature connection", restart your docker instance and wait a minute until
> your ScyllaDB connection be established. 

... and also will create the **php-workspace**, where your web server will run. You can access them with the `docker`
command.

### Useful Commands

Here's a list of everything that you can execute and make your own research through the application.

#### PHP Application Commands

These commands you can execute by `entering the container` or through `docker exec` remotely:


##### Entering App Container:
````shell
$ docker exec -it workspace-php php scylla bash
root@14a656685517:/var/www# 
root@14a656685517:/var/www# php scylla help
````

##### Initializing Database:
````shell
$ docker exec -it workspace-php php scylla migrate
[INFO] Fetching Migrations... 
[INFO] Migrated: /var/www/migrations/1-create_keyspace.cql 
[INFO] Migrated: /var/www/migrations/2-create_owner_table.cql 
[INFO] Migrated: /var/www/migrations/3-create_pets_table.cql 
[INFO] Migrated: /var/www/migrations/4-create_sensors_table.cql 
[INFO] Migrated: /var/www/migrations/5-create_measurements_table.cql 
[INFO] Migrated: /var/www/migrations/6-create_sensor_avg_table.cql 
[INFO] Done :D 
````

##### Starting Web Server:
````shell
$ php scylla serve
[INFO] CarePet Web started!
[INFO] Development Server: http://0.0.0.0:8000
[Thu Jan  5 17:32:01 2023] PHP 7.4.33 Development Server (http://0.0.0.0:8000) started
````

#### ScyllaDB Commands

##### Running Nodetool:
`````shell
$ docker exec -it carepet-scylla1 nodetool status
=======================
Datacenter: datacenter1
=======================
Status=Up/Down
|/ State=Normal/Leaving/Joining/Moving
--  Address    Load       Tokens       Owns    Host ID                               Rack
UN  10.10.5.2  212 KB     256          ?       f6121e15-48df-4b31-b725-3ad2795b8b94  rack1
UN  10.10.5.3  1.06 MB    256          ?       871795f3-67d2-47ba-83ef-15714b89c02a  rack1
UN  10.10.5.4  1.06 MB    256          ?       cbe74a63-2cf4-41c2-bf7f-c831c0d2689f  rack1
`````

##### Running Container Shell:

````shell
$ docker exec -it carepet-scylla1 bash

   _____            _ _       _____  ____
  / ____|          | | |     |  __ \|  _ \
 | (___   ___ _   _| | | __ _| |  | | |_) |
  \___ \ / __| | | | | |/ _` | |  | |  _ <
  ____) | (__| |_| | | | (_| | |__| | |_) |
 |_____/ \___|\__, |_|_|\__,_|_____/|____/
               __/ |
              |___/
Nodetool:
        nodetool help
CQL Shell:
        cqlsh
More documentation available at:
        http://www.scylladb.com/doc/

root@7e2b1b94389b:/#
````
##### Inspecting a Container
_You can inspect any node by means of the `docker inspect` command as follows. For example:_

````shell
$ docker inspect carepet-scylla1
[
    {
        "Id": "7e2b1b94389b36c494093db8e119c2b8c5167339f20e03d9bfa070e8e46f8430",
        "Created": "2023-01-05T17:36:59.038609825Z",
        "Path": "/docker-entrypoint.py",
        "Args": [
            "--smp",
            "1"
        ],
        "State": {
            "Status": "running",
            "Running": true,
            "Paused": false,
            "Restarting": false
            ...
        }
    }
]
````

###### Get Node IP Address:
````shell
$ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1
10.10.5.2
````

##### Entering CQLSH (Database)

````shell
$ docker exec -it carepet-scylla1 cqlsh
Connected to  at 10.10.5.2:9042.
[cqlsh 5.0.1 | Cassandra 3.0.8 | CQL spec 3.3.1 | Native protocol v4]
Use HELP for help.
cqlsh>
````

````sql
cqlsh> DESCRIBE KEYSPACES
carepet  system_schema  system_auth  system  system_distributed  system_traces
````

````sql
cqlsh> USE carepet;
cqlsh:carepet> DESCRIBE TABLES
pet  sensor_avg  gocqlx_migrate  measurement  owner  sensor
````
````sql
cqlsh:carepet> DESCRIBE TABLE pet
CREATE TABLE carepet.owner (
                               owner_id uuid PRIMARY KEY,
                               address text,
                               name text
) WITH bloom_filter_fp_chance = 0.01
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
````

To start pet collar simulation execute the following in the separate terminal:

Expected output:

````log
info: Welcome to the Pet collar simulator
info: New owner # aab6069c-b224-416f-95b5-0f418b08fd62
info: New pet # a5f36a41-249f-4689-9bea-5f4ac65ea69a
info: New sensor # 6d509c73-01b9-4006-8398-3891c7c0f23f
info: New sensor # 6700676b-9b22-43d3-82b3-d56a31f8559d
info: sensor # 6d509c73-01b9-4006-8398-3891c7c0f23f type L new measure 33.446360291975644 ts 2022-02-25T18:26:59.487Z
info: sensor # 6700676b-9b22-43d3-82b3-d56a31f8559d type L new measure 34.20680666491499 ts 2022-02-25T18:26:59.488Z
info: sensor # 6d509c73-01b9-4006-8398-3891c7c0f23f type L new measure 33.36286546273347 ts 2022-02-25T18:27:04.489Z
...
````

In a minute (a `--buffer-interval`) you will see a data push (`Pushing data`) log line.
That means that the collar has been pushed buffered measurements to the app.

Write down the pet Owner ID (ID is something after the `#` sign without trailing spaces).
To start REST API service execute the following in the separate terminal:

````shell
$ docker exec -it workspace-php php scylla serve
````

Expected output:

````log
[INFO] CarePet Web started! 
[INFO] Development Server: http://0.0.0.0:8000 
````

## Structure

Package structure is as follows:

| Name            | Purpose                                             |
|-----------------|-----------------------------------------------------|
| /src/Core       | Application Core (Connection, abstractions etc)     |
| /src/Owner      | Owner Domain: DTO, Repository, Services             |

## Implementation

Collars are small devices that attach to pets and collect data
with the help of different sensors. After the data is collected
it may be delivered to the central database for the analysis and
health status checking.

Collar code sits in the `/src/cmd/sensor` and uses [Cassandra NodeJS](https://github.com/datastax/nodejs-driver)
driver to connect to the database directly and publish its data.
Collar gathers sensors measurements, aggregates data in a buffer and
sends it every hour.

Overall all applications in this repository use [Cassandra NodeJS](https://github.com/datastax/nodejs-driver) for:

- Relational Object Mapping (ORM)
- Build Queries
- Migrate database schemas

The web application REST API server resides in `/src/index.js` and uses
[ExpressJS](https://expressjs.com/). API handlers reside in `/src/api`.
Most of the queries are reads.

The application is capable of caching sensor measurements data
on hourly basis. It uses lazy evaluation to manage `sensor_avg`.
It can be viewed as an application-level lazy-evaluated
materialized view.

The algorithm is simple and resides in `/src/api/avg.js`:

- read `sensor_avg`
- if no data, read `measurement` data, aggregate in memory, save
- serve request

## Architecture

    Pet --> Sensor --> ScyllaDB <-> REST API Server <-> User

## How to start a new project with NodeJS

[Install NodeJS and NPM](https://nodejs.org/en/download/). Create a repository. Clone it. Execute inside of
your repository:

    $ mkdir project_name && cd project_name && npm init

Now install the driver by running:

    $ npm install cassandra-driver

Now you are ready to connect to the database and start working.
To connect to the database, do the following:

```javascript
const client = new cassandra.Client({
    contactPoints: ['127.0.0.1:9042'],
    authProvider: new cassandra.auth.PlainTextAuthProvider('', ''),
    localDataCenter: 'datacenter1',
    keyspace: 'keyspace',
});

await client.connect();
```

Now you can issue CQL commands:

```javascript
const {rows} = await client.execute('SELECT a, b, c FROM ks.t');
console.log(JSON.stringify(rows));
```

Or save models:

```javascript
const toInsert = 12345;
await client.execute('INSERT INTO keyspace.table (a) VALUES(?)', [toInsert]);
```

For more details, check out `/src/api`, `/src/db.js` and `/src/model.js` files.

## Links

- https://hub.docker.com/r/scylladb/scylla
- https://github.com/datastax/nodejs-driver
