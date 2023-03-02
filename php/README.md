# Care Pet ScyllaDB IoT example

This example project demonstrates a generic IoT use case for ScyllaDB in PHP.

Here you will find a list of possible drivers to integrate with.

| PHP Version | Driver                                                                    |
|-------------|---------------------------------------------------------------------------|
| PHP 7.1     | [DataStax PHP Driver](https://github.com/datastax/php-driver)             |
| PHP 8.2 [x] | [ScyllaDB PHP Driver (dev)](https://github.com/he4rt/scylladb-php-driver) |

The documentation for this application and the guided exercise is [here](../docs).

## Quick Start

The application allows the tracking of the pets health indicators, and it consists in a CLI of three parts:

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

Docker-compose will spin up three nodes which are:

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

... and it will also create the **php-workspace**, where your web server will run. You can access them with the `docker`
command.

### Useful Commands

Here's a list of everything that you can execute and make your own research through the application.

#### PHP Application Commands

These commands you can execute by `entering the container` or through `docker exec` remotely:

##### Entering App Container:

````shell
$ docker exec -it workspace-php bash
root@14a656685517:/var/www# php scylla migrate
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
$ docker exec -it workspace-php php scylla serve
[INFO] CarePet Web started!
[INFO] Development Server: http://0.0.0.0:8000
[Thu Jan  5 17:32:01 2023] PHP 7.4.33 Development Server (http://0.0.0.0:8000) started
````

##### Simulate Environment Sensors:

````shell
$ docker exec -it workspace-php php scylla simulate
[INFO] Starting Sensor simulator... 
[INFO] Batch: 0
[INFO] Owner 593dec12-6bea-3c93-8f49-26d8b6d589b1 
[INFO] Pet: 14d9f304-5600-34af-8622-3d4505d617d7 | Owner 593dec12-6bea-3c93-8f49-26d8b6d589b1
[INFO] Sensor: 869bd01e-e0ba-364f-bbfb-8c7c496a3318 (R) | Pet 14d9f304-5600-34af-8622-3d4505d617d7 
[INFO] Sensor: c86f63b0-1439-3404-8750-b71b90a685cb (L) | Pet 14d9f304-5600-34af-8622-3d4505d617d7 
[INFO] Sensor: e0550426-8832-3d17-9025-77726b3009c5 (P) | Pet 14d9f304-5600-34af-8622-3d4505d617d7 
[INFO] Sensor: bf960c81-8e0f-3012-b50d-18596b50db18 (P) | Pet 14d9f304-5600-34af-8622-3d4505d617d7 
[INFO] Sensor: 933245de-812e-34e4-8d50-2ab072726217 (T) | Pet 14d9f304-5600-34af-8622-3d4505d617d7 
[INFO] Pet: 319ec566-d6b0-3868-ac5e-76253ee7c236 | Owner 593dec12-6bea-3c93-8f49-26d8b6d589b1
[INFO] ...
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

_You can inspect any node using the `docker inspect` command as follows. For example:_

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
cqlsh
> DESCRIBE KEYSPACES
carepet  system_schema  system_auth  system  system_distributed  system_traces
````

```sql
cqlsh
> USE carepet;
cqlsh
:carepet> DESCRIBE TABLES
pet  sensor_avg  gocqlx_migrate  measurement  owner  sensor
```

```sql
cqlsh
:carepet> DESCRIBE TABLE pet
CREATE TABLE carepet.owner
(
    owner_id uuid PRIMARY KEY,
    address  text,
    name     text
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

cqlsh
:carepet> exit
```

## Architecture

Pet --> Sensor --> ScyllaDB <-> REST API Server <-> User

- https://hub.docker.com/r/scylladb/scylla
- https://github.com/he4rt/scylladb-php-driver
