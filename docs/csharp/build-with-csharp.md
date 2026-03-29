# Build an IoT App with CSharp

## Introduction

In this section, we will walk you through the CarePet commands and explain the code behind them.

As explained in [Getting Started with CarePet](/getting-started.md), the project is structured as follows:
- Migrate (CarePet.Migrate) - Creates the CarePet keyspace and tables.
- Collar (CarePet.Sensor) - Simulates a pet's collar by generating the pet's health data and pushing the data into the storage.
- Server (CarePet.Server.App) - REST API service for tracking the pets’ health state.

## Prerequisites:

- [.NET 8.0 SDK](https://dotnet.microsoft.com/download/dotnet/8.0)
- [docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

## Setup

Clone the repository and change to the csharp directory:

```
git clone git@github.com:scylladb/care-pet.git
cd csharp
```

To run a local ScyllaDB cluster consisting of three nodes with
the help of `docker` and `docker-compose` execute:

    $ docker-compose up -d

Docker-compose will spin up three nodes: `csharp-carepet-scylla1-1`, `csharp-carepet-scylla2-1`
and `csharp-carepet-scylla3-1`. You can access them with the `docker` command.

## Migrate

The `dotnet run --project CarePet.Migrate.csproj --hosts $NODE1 --datacenter datacenter1` command executes the main function in the `Migrate` class located in `Migrate.cs`. The function creates the keyspace and tables used by the collar and server services.

The following code in the `Migrate.cs` file calls the `createKeyspace` , `createSchema` , and `printMetadata` functions.

```
public static void Main(string[] args)
{
    var config = Config.Parse(new Config(), args);

    var client = new Migrate(config);
    client.CreateKeyspace();
    client.CreateSchema();
    client.PrintMetadata();
}
```

Let's break down the code line by line.

The `config` object parses the arguments passed in the migrate command. In our case it's `hosts` and `datacenter`. The `hosts` argument expects the IP address of one of the nodes. The `datacenter` argument is `datacenter1` by default but could be different if you use Scylla Cloud. The command also accepts `username` and `password` arguments if required.

The `CreateKeyspace` function creates a new `ISession`, then executes the following CQL query stored in the `Resources/care-pet-keyspace.cql` file:

```
public void CreateKeyspace()
{
    LOG.LogInformation("Creating keyspace carepet...");
    using (var session = Connect())
    {
        var cql = Config.GetResource("care-pet-keyspace.cql");
        if (!string.IsNullOrWhiteSpace(cql))
        {
            session.Execute(cql);
        }
    }
    LOG.LogInformation("Keyspace carepet created successfully");
}
```

```
CREATE KEYSPACE IF NOT EXISTS carepet
WITH replication = {
    'class': 'NetworkTopologyStrategy',
    'datacenter1': 3
} AND durable_writes = true;
```

The CQL query above creates a new keyspace named carepet, with `NetworkTopologyStrategy` as replication strategy and a replication factor of 3.
See [Scylla University](https://university.scylladb.com/courses/data-modeling/lessons/basic-data-modeling-2/topic/keyspace/) for more information about keyspaces and replication.

The `CreateSchema` function opens a new session with the `carepet` keyspace and creates the following tables in the carepet keyspace using the CQL file located in `Resources/care-pet-ddl.cql`:

-   `owner`
-   `pet`
-   `sensor`
-   `measurement`
-   `sensor_avg`

```
public void CreateSchema()
{
    LOG.LogInformation("Creating tables...");
    using (var session = Keyspace())
    {
        var ddl = Config.GetResource("care-pet-ddl.cql");
        if (!string.IsNullOrWhiteSpace(ddl))
        {
            var statements = ddl.Split(';')
                                .Select(s => s.Trim())
                                .Where(s => !string.IsNullOrEmpty(s));
            foreach (var cql in statements)
            {
                session.Execute(cql);
            }
        }
    }
}
```

The `PrintMetadata` function will print the metadata related to the `carepet` keyspace and confirm that the tables are properly created.

You can check the database structure with:

```
$ docker exec -it csharp-carepet-scylla1-1 cqlsh
cqlsh> USE carepet;
cqlsh:carepet> DESCRIBE TABLES
cqlsh:carepet> DESCRIBE TABLE pet
```

You should expect the following result:

```
CREATE TABLE carepet.pet (
owner_id uuid,
pet_id uuid,
chip_id text,
species text,
breed   text,
color   text,
gender  text,
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
AND dclocal_read_repair_chance = 0.1
AND default_time_to_live = 0
AND gc_grace_seconds = 864000
AND max_index_interval = 2048
AND memtable_flush_period_in_ms = 0
AND min_index_interval = 128
AND read_repair_chance = 0.0
AND speculative_retry = '99.0PERCENTILE';
```

## Sensor

The sensor service simulates the collar's activity. You can use the following command to run the sensor service:

```
$ dotnet build CarePet.Sensor.csproj
$ dotnet run --project CarePet.Sensor.csproj --hosts $NODE1 --datacenter datacenter1 --measure 00:01:00 --buffer-interval 00:01:00
```

The above command executes `Sensor.cs` and the following `Main` function:

```
public static void Main(string[] args)
{
    var config = SensorConfig.Parse(args);
    var client = new Sensor(config);
    client.Save();
    client.Run();
}
```

First, we create a client object, an instance of the Sensor class. Like in the `Migrate` class, we parse args using the `SensorConfig.Parse()` method to connect to the database.

In the `Sensor` constructor, a random ID is attributed to the `owner`, `pet`, and `sensors`.

```
public Sensor(SensorConfig config)
{
    _config = config;
    _owner = Owner.Random();
    _pet = Pet.Random(_owner.OwnerId);
    _sensors = new CarePet.Model.Sensor[Enum.GetValues(typeof(SensorType)).Length];

    var sensorTypes = Enum.GetValues(typeof(SensorType)).Cast<SensorType>().ToArray();
    for (int i = 0; i < _sensors.Length; i++)
    {
        _sensors[i] = new CarePet.Model.Sensor(_pet.PetId, Guid.NewGuid(), SensorTypeExtensions.GetTypeCode(sensorTypes[i]));
    }
}
```

The `client.Save()` method connects to the datbase and saves the generated `owner`, `pet`, and the `sensors`.

```
private void Save()
{
    using (var session = Keyspace())
    {
        var mapper = new Mapper(session);
        LOG.LogInformation($"owner = {_owner}");
        LOG.LogInformation($"pet = {_pet}");

        mapper.Owner().Create(_owner);
        mapper.Pet().Create(_pet);

        foreach (var s in _sensors)
        {
            LOG.LogInformation($"sensor = {s}");
            mapper.Sensor().Create(s);
        }
    }
}
```

The `client.Run()` generates random data and pushes it to the database. In this code, we are using `PreparedStatement` to define the query and `BatchStatement` to run multiple queries at the same time. See the [ScyllaDB CSharp Driver documentation](https://csharp-driver.docs.scylladb.com/stable/features/components/core/statements/prepared/index.html) for details on `PreparedStatement`.

```
private void Run()
{
    using (var session = Keyspace())
    {
        var prepared = session.Prepare("INSERT INTO measurement (sensor_id, ts, value) VALUES (?, ?, ?)");
        var ms = new List<Measure>();
        var prev = DateTimeOffset.UtcNow;

        while (true)
        {
            while ((DateTimeOffset.UtcNow - prev) < _config.BufferInterval)
            {
                if (!Sleep(_config.Measurement))
                    return;

                foreach (var s in _sensors)
                {
                    var m = ReadSensorData(s);
                    ms.Add(m);
                    LOG.LogInformation(m.ToString());
                }
            }

            var elapsed = DateTimeOffset.UtcNow - prev;
            var intervals = elapsed.Ticks / _config.BufferInterval.Ticks;
            prev = prev.AddTicks(intervals * _config.BufferInterval.Ticks);

            LOG.LogInformation("pushing data");

            var batch = new BatchStatement();
            foreach (var m in ms)
            {
                batch.Add(prepared.Bind(m.SensorId, m.Ts.UtcDateTime, m.Value));
            }

            session.Execute(batch);
            ms.Clear();
        }
    }
}
```

## Server

The server service is a REST API for tracking the pets’ health state. The service allows you to query the database via HTTP.

Run the following commands to start the server:

```
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' csharp-carepet-scylla1-1)
$ dotnet run --project CarePet.csproj --hosts $NODE1 --datacenter datacenter1
```

In the care-pet example, run:

  `$ curl http://127.0.0.1:8000/api/owner/{id}`.

  You can expect the following response:

```

[{"address":"home","age":57,"name":"tlmodylu","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360","pet_id":"a52adc4e-7cf4-47ca-b561-3ceec9382917","weight":5}]

```

The controller is defined in `ModelController.cs`, and implements the GET methods to access owners, pets and sensors data.

The server also aggregates the data and saves it to the database in the sensor_avg table:

```
private void SaveAggregate(Guid sensorId, List<float> data, int prevSize, DateTime day, DateTime nowUtc)
{
    bool sameDate = nowUtc.Date == day.Date;
    int currentHour = nowUtc.Hour;

    for (int hour = prevSize; hour < data.Count; hour++)
    {
        if (sameDate && hour >= currentHour)
            break;

        _mapper.SensorAvg().CreateAsync(new SensorAvg(sensorId, day, hour, data[hour]));
    }
}
```

## Resources

* [Scylla CSharp driver documentation](https://csharp-driver.docs.scylladb.com/stable/index.html)
* [ScyllaDB CSharp driver on Github](https://github.com/scylladb/csharp-driver/)
