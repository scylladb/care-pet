# Care Pet ScyllaDB PHP IoT example

This example project demonstrates a generic IoT use case for ScyllaDB in PHP.

Here you will find a list of possible drivers to integrate with.

| PHP Version  | Driver                                                                              |
|--------------|-------------------------------------------------------------------------------------|
| PHP 7.1      | [DataStax PHP Driver](https://github.com/datastax/php-driver)                       |
| PHP 8.2  [x] | [ScyllaDB PHP Driver (community)](https://github.com/qkdreyer/cassandra-php-driver) |

You will need to build the driver following the instructions of each repository. We strongly recommend that you go for
PHP 8.x since this project still being maintained and developed by community itself.

The documentation for this application and the guided exercise is [here](getting-started.md).

## Quick Start

The application allows the tracking of the pets health indicators and it consist in a CLI of three parts:

| Command             | Description                                                |
|---------------------|------------------------------------------------------------|
| php scylla migrate  | creates the `carepet` keyspace and tables                  |
| php scylla simulate | generates a pet health data and pushes it into the storage |
| php scylla serve    | REST API service for tracking pets health state            |

Prerequisites:

- [docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

## Setup

To run a local **ScyllaDB cluster** consisting of three nodes and the **PHP Environment** with
the help of `docker` and `docker-compose` execute:

```shell
$ docker-compose up -d
```    

Docker-compose will spin up three nodes which are:

- carepet-scylla1
- carepet-scylla2
- carepet-scylla3

If you want to see your containers running, run the `docker ps` command, and you should see something like this:

```shell
$ docker ps
CONTAINER ID   IMAGE                    COMMAND                  CREATED       STATUS       PORTS                                                                      NAMES
4e351dfe3987   scylladb/scylla          "/docker-entrypoint.…"   1 minute ago   Up 1 minute   22/tcp, 7000-7001/tcp, 7199/tcp, 9042/tcp, 9160/tcp, 9180/tcp, 10000/tcp   carepet-scylla2
9e7e4d3992df   scylladb/scylla          "/docker-entrypoint.…"   1 minute ago   Up 1 minute   22/tcp, 7000-7001/tcp, 7199/tcp, 9042/tcp, 9160/tcp, 9180/tcp, 10000/tcp   carepet-scylla3
7e2b1b94389b   scylladb/scylla          "/docker-entrypoint.…"   1 minute ago   Up 1 minute   22/tcp, 7000-7001/tcp, 7199/tcp, 9042/tcp, 9160/tcp, 9180/tcp, 10000/tcp   carepet-scylla1
```

> If you have any error regarding "premature connection", restart your docker instance and wait a minute until
> your ScyllaDB connection be established.

### Useful Commands

Here's a list of everything that you can execute and make your own research through the application.

#### PHP Application Commands

These commands you can execute by `entering the container` or through `docker exec` remotely:

##### Configuring the Environment

Make a copy of `.env.example` and name it `.env`. This file will store your application secrets.

````shell
$ cp .env.example .env
````

By default, the config to connect on your local ScyllaDB instances will be ready to use.

```
# Development
DB_KEYSPACE="carepet"
DB_NODES="localhost"
DB_USERNAME=""
DB_PASSWORD=""
DB_PORT=9042

# Production (Cloud)
#DB_KEYSPACE="carepet"
#DB_NODES="node-0.aws_sa_east_1.c106d1ac5f3117a20bf0.clusters.scylla.cloud"
#DB_USERNAME="scylla"
#DB_PASSWORD="p50bonFq8cuxwXS"
#DB_PORT=9042
```

> If you want to use ScyllaDB Cloud, remember to change at your keyspace the **Replication Factor** related to 
> for each environment.

##### Initializing Keyspace:

First, let's create our keyspace using CQLSH.

```shell
$ docker exec -it carepet-scylla1 cqlsh
Connected to  at 10.10.5.2:9042.
[cqlsh 5.0.1 | Cassandra 3.0.8 | CQL spec 3.3.1 | Native protocol v4]
Use HELP for help.
cqlsh> CREATE KEYSPACE IF NOT EXISTS carepet WITH replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': '2' };
cqlsh>
```

Then you can run the CLI command to migrate all your tables inside the keyspace.

```shell
$ php scylla migrate
[INFO] Fetching Migrations... 
[INFO] Migrated: /var/www/migrations/1-create_keyspace.cql 
[INFO] Migrated: /var/www/migrations/2-create_owner_table.cql 
[INFO] Migrated: /var/www/migrations/3-create_pets_table.cql 
[INFO] Migrated: /var/www/migrations/4-create_sensors_table.cql 
[INFO] Migrated: /var/www/migrations/5-create_measurements_table.cql 
[INFO] Migrated: /var/www/migrations/6-create_sensor_avg_table.cql 
[INFO] Done :D 
```

##### Starting Web Server:

```shell
$ php scylla serve
[INFO] CarePet Web started!
[INFO] Development Server: http://0.0.0.0:8000
[Thu Jan  5 17:32:01 2023] PHP 7.4.33 Development Server (http://0.0.0.0:8000) started
```

##### Simulate Environment Sensors:

```shell
$ php scylla simulate
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
```

````php
final class SimulateCommand extends AbstractCommand
{

    public function __construct(
        private readonly OwnerRepository $ownerRepository,
        private readonly PetRepository    $petRepository,
        private readonly SensorRepository $sensorRepository
    )
    {
    }

    const AMOUNT_BASE = 50000;

    public function __invoke(array $args = []): int
    {
        $this->info('Starting Sensor simulator...');
        foreach (range(0, self::AMOUNT_BASE) as $i) {
            $this->info("Batch: " . $i);
            [$ownerDTO, $petsDTO] = $this->generateFakeData();

            $this->ownerRepository->create($ownerDTO);
            $this->info(sprintf('Owner %s', $ownerDTO->id));

            $petsDTO->each(function ($petDTO) {
                $this->info(sprintf('Pet: %s | Owner %s', $petDTO->id->uuid(), $petDTO->ownerId));
                $this->petRepository->create($petDTO);

                SensorFactory::makeMany(5, ['pet_id' => $petDTO->id])
                    ->each($this->handleSensors());
            });
        }
        $this->info('Done :D');

        return self::SUCCESS;
    }

    private function generateFakeData(): array
    {
        $ownerDTO = OwnerFactory::make();
        $petsDTO = PetFactory::makeMany(5, ['owner_id' => $ownerDTO->id]);

        return [$ownerDTO, $petsDTO];
    }

    private function handleSensors(): Closure
    {
        return function (SensorDTO $sensorDTO) {
            $this->sensorRepository->create($sensorDTO);
            $this->info(sprintf(
                'Sensor: %s (%s) | Pet %s',
                $sensorDTO->id,
                $sensorDTO->type->name,
                $sensorDTO->petId
            ));
        };
    }
}
````

##### Inserting Data

You can use `Cassandra::cluster()` and setup your cluster.

````php
use Cassandra;
use Cassandra\Cluster;
use Cassandra\Cluster\Builder;
use Cassandra\FutureRows;
use Cassandra\Session;
use Cassandra\SimpleStatement;

class Connector
{
    public Builder $connectionBuilder;
    public Cluster $cluster;
    public Session $session;
    public SimpleStatement $query;

    const BASE_TIMEOUT = 10;

    public function __construct(array $config)
    {
        $this->connectionBuilder = Cassandra::cluster()
            ->withContactPoints($config['nodes'])
            ->withDefaultConsistency($config['consistency_level'])
            ->withPort($config['port']);

        if (!empty($config['username'] && !empty($config['password']))) {
            $this->connectionBuilder = $this->connectionBuilder->withCredentials($config['username'], $config['password']);
        }
        $this->cluster = $this->connectionBuilder->build();

        $this->session = $this->cluster->connect($config['keyspace']);
    }

    public function setKeyspace(string $keyspace = ''): self
    {
        $this->session->close(self::BASE_TIMEOUT);
        $this->session = $this->cluster->connect($keyspace);

        return $this;
    }

    public function prepare(string $query): self
    {
        $this->query = new SimpleStatement($query);

        return $this;
    }

    public function execute(): FutureRows
    {
        return $this->session->executeAsync($this->query, []);
    }
}

````

````php
use App\Core\Entities\AbstractDTO;
use Cassandra\Rows;

abstract class AbstractRepository
{
    public string $table = '';

    public string $primaryKey = '';

    public Connector $connection;

    public array $keys = [];

    public function __construct(Connector $connector)
    {
        $this->connection = $connector;
    }

    public function getById(string $id): Rows
    {
        $query = sprintf("SELECT * FROM %s WHERE %s = %s", $this->table, $this->primaryKey, $id);

        return $this->connection
            ->prepare($query)
            ->execute()
            ->get(Connector::BASE_TIMEOUT);
    }

    public function all(): Rows
    {
        return $this->connection
            ->prepare(sprintf('SELECT * FROM %s', $this->table))
            ->execute()
            ->get(Connector::BASE_TIMEOUT);
    }

    public function create(AbstractDTO $dto): void
    {
        $keys = array_keys($dto->toDatabase());
        $dataValues = array_values($dto->toDatabase());

        foreach ($dataValues as $key => $value) {
            if (is_string($value) && !in_array($keys[$key], $this->keys)) {
                $value = addslashes($value);
                $dataValues[$key] = "'$value'";
            }
        }

        $query = sprintf(
            "INSERT INTO %s (%s) VALUES (%s)",
            $this->table,
            implode(', ', $keys),
            implode(', ', $dataValues)
        );


        $this->connection
            ->prepare($query)
            ->execute();
    }
}
````

#### ScyllaDB Commands

##### Running Nodetool:

```shell
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
```

##### Running Container Shell:

```shell
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
```

##### Inspecting a Container

_You can inspect any node by means of the `docker inspect` command as follows. For example:_

```shell
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
```

###### Get Node IP Address:

```shell
$ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1
10.10.5.2
```

### Resources

* [ScyllaDB PHP driver for PHP 8.2.x  on Github (maintained by community)](https://github.com/he4rt/scylladb-php-driver)

* [Driver for PHP 7.1 (third-party, not actively maintained](https://github.com/datastax/php-driver)