## Build an IoT App with Java

### Architecture

In this section, we will walk you through the CarePet commands and explain the code behind them.
The project is structured as follows:

-   Migrate (`com.carepet.Migrate`) - Creates the `carepet` keyspace and tables.
-   Collar (`com.carepet.Sensor`) - Generates pet health data and pushes it into the storage.
-   Web app (`com.carepet.server.App`) - REST API service for tracking pets' health state.

### Migrate

The `./bin/migrate.sh --hosts $NODE1 --datacenter datacenter1` command executes the main function in the `Migrate` class located `Migrate.java`. The function creates the keyspace and tables used by the collar and server services.

The following code in the `Migrate.java` file calls the `createKeyspace` , `createSchema` , and `printMetadata` functions.

```
public static void main(String[] args) {
	final Config config = Config.parse(new Config(), args);

	final Migrate client = new Migrate(config);
	client.createKeyspace();
	client.createSchema();
	client.printMetadata();
}
```

Let's break down the code line by line.

The `config` object parses the arguments passed in the migrate command. In our case it's `hosts` and `datacenter`. The `hosts` argument expects the IP address of one of the nodes. The `datacenter` argument is `datacenter1` by default but could be different if you use Scylla Cloud. The command also accepts `username` and `password` arguments if required.

The `createKeyspace` function creates a new `CqlSession`, then executes the following CQL query stored in the `resources/care-pet-keyspace.cql` file:

```
public void createKeyspace() {
	LOG.info("creating keyspace...");
	try (CqlSession session = connect()) {
			session.execute(Config.getResource("care-pet-keyspace.cql"));
	}
}
```

```
CREATE KEYSPACE IF NOT EXISTS carepet WITH replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': '3' } AND durable_writes = TRUE;
```

The CQL query above creates a new keyspace named carepet, with `NetworkTopologyStrategy` as replication strategy and a replication factor of 3.
See [Scylla University](https://university.scylladb.com/courses/data-modeling/lessons/basic-data-modeling-2/topic/keyspace/) for more information about keyspaces and replication.

The `createSchema` function opens a new session with the `carepet` keyspace and creates the following tables in the carepet keyspace using the CQL file located in `resources/care-pet-ddl.cql`:

-   `owner`
-   `pet`
-   `sensor`
-   `measurement`
-   `sensor_avg`

```
public void createSchema() {
	LOG.info("creating table...");
	try (CqlSession session = keyspace()) {
	    for (String cql : Config.getResource("care-pet-ddl.cql").split(";")) {
		session.execute(cql);
	    }
	}
}
```

The `printMetadata` function will print the metadata related to the `carepet` keyspace and confirm that the tables are properly created.

You can check the database structure with:

```
$ docker exec -it carepet-scylla1 cqlsh
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

### Sensor

The sensor service simulates the collar's activity. You can use the following command to run the sensor service:

```
$ mvn package
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
$ ./bin/sensor.sh --hosts $NODE1 --datacenter datacenter1 --measure PT1M --buffer-interval PT1M
```

The above command executes `Sensor.java` and the following `main` function:

```
public static void main(String[] args) {
        final Sensor client = new Sensor(Config.parse(new SensorConfig(), args));
        client.save();
        client.run();
    }
}
```

First, we create a client object, an instance of the Sensor class. Like in the `Migrate` class, we parse args using the `Config.parse()` method to connect to the database.

In the `Sensor` constructor, a random ID is attributed to the `owner`, `pet`, and `sensors`.

```
public Sensor(SensorConfig config) {
	this.config = config;

	this.owner = Owner.random();
	this.pet = Pet.random(this.owner.getOwnerId());
	this.sensors = new com.carepet.model.Sensor[SensorType.values().length];
	for (int i = 0; i < this.sensors.length; i++) {
			this.sensors[i] = com.carepet.model.Sensor.random(this.pet.getPetId());
	}
}
```

The `client.save()` method connects to the datbase and saves the generated `owner`, `pet`, and the `sensors`.

```
private void save() {
	try (CqlSession session = keyspace()) {
			// Connect to the database
			Mapper m = Mapper.builder(session).build();

			LOG.info("owner = " + owner);
			LOG.info("pet = " + pet);

			m.owner().create(owner);
			m.pet().create(pet);

			for (com.carepet.model.Sensor s : sensors) {
					LOG.info("sensor = " + s);

					m.sensor().create(s);
			}
	}
}
```

The `client.run()` generates random data and pushes it to the database. In this code, we are using `PreparedStatement` to define the query and `BatchStatementBuilder` to run multiple queries at the same time. See the [Scylla Java Driver documentation] (https://java-driver.docs.scylladb.com/stable/manual/statements/prepared/) for details on `PreparedStatement`.

```
private void run() {
	try (CqlSession session = keyspace()) {
			PreparedStatement statement = session.prepare("INSERT INTO measurement (sensor_id, ts, value) VALUES (?, ?, ?)");
			BatchStatementBuilder builder = new BatchStatementBuilder(BatchType.UNLOGGED);

			List<Measure> ms = new ArrayList<>();
			Instant prev = Instant.now();

			while (true) {
					while (Duration.between(prev, Instant.now()).compareTo(config.bufferInterval) < 0) {
							if (!sleep(config.measurement)) {
									return;
							}

							for (com.carepet.model.Sensor s : sensors) {
									Measure m = readSensorData(s);
									ms.add(m);
									LOG.info(m.toString());
							}
					}

					prev = prev.plusMillis((Duration.between(prev, Instant.now()).toMillis() / config.bufferInterval.toMillis()) * config.bufferInterval.toMillis());

					LOG.info("pushing data");
					// this is simplified example of batch execution. standard
					// best practice is to batch values that end up in the same partition:
					// https://www.scylladb.com/2019/03/27/best-practices-for-scylla-applications/
					for (Measure m : ms) {
							builder = builder.addStatement(statement.bind(m.getSensorId(), m.getTs(), m.getValue()));
					}

					session.execute(builder.build());

					builder.clearStatements();
					ms.clear();
			}
	}
}
```

### Server

The server service is a REST API for tracking the pets’ health state. The service allows you to query the database via HTTP.

Run the following commands to start the server:

```
$ mvn package
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
$ ./bin/server.sh --hosts $NODE1 --datacenter datacenter1
```

In the care-pet example, run:

  `$ curl http://127.0.0.1:8000/api/owner/{id}`.
  
  You can expect the following response:

```

[{"address":"home","age":57,"name":"tlmodylu","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360","pet_id":"a52adc4e-7cf4-47ca-b561-3ceec9382917","weight":5}]

```

The controller is defined in `ModelController.java`, and implements the GET methods to access owners, pets and sensors data.

The server also aggregates the data and saves it to the database in the sensor_avg table:

```

// saveAggregate saves the result monotonically sequentially to the database
private void saveAggregate(UUID sensorId, List<Float> data, int prevAvgSize, LocalDate day, LocalDateTime now) {
// if it's the same day, we can't aggregate current hour
boolean sameDate = now.getDayOfYear() == day.getDayOfYear();
int current = now.getHour();

        for (int hour = prevAvgSize; hour < data.size(); hour++) {
            if (sameDate && hour >= current) {
                break;
            }

            mapper.sensorAvg().create(new SensorAvg(sensorId, day, hour, data.get(hour)));
        }
    }

```

```
