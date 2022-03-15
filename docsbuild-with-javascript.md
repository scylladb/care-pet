## Build an IoT App with JavaScript

### Architecture

This section will walk through and explain the code for the different commands.
As explained in the Getting Started page, the project is structured as follow:

- migrate (`npm run migrate`) - creates the `carepet` keyspace and tables
- collar (`npm run sensor`) - generates a pet health data and pushes it into the storage
- web app (`npm run dev`) - REST API service for tracking pets health state

## Quick Start

Prerequisites:

- [NodeJS](https://nodejs.org/en/) tested with v17.0.1
- [NPM](https://www.npmjs.com/) tested with v8.1.0
- [docker](https://www.docker.com/) (not required if you use Scylla Cloud)
- [docker-compose](https://docs.docker.com/compose/) (not required if you use Scylla Cloud)

Make sure to install all NodeJS dependencies with:

    $ npm install
    
### Use ScyllaDB on your local machine

To run a local ScyllaDB cluster consisting of three nodes with
the help of `docker` and `docker-compose` execute:

    $ docker-compose up -d

Docker-compose will spin up three nodes: `carepet-scylla1`, `carepet-scylla2`
and `carepet-scylla3`. You can access them with the `docker` command.

Execute the following nodetool command:

    $ docker exec -it carepet-scylla1 nodetool status

### Migrate

#### Using ScyllaDB on your local machine

Once all the nodes are in UN - Up Normal status, run the below commands:

The below command allows you to get node IP address:

```
docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1
```

The run the following commands to execute the migrate main function.

```
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
npm run migrate -- --hosts $NODE1
```
#### Using Scylla Cloud

If you are using Scylla Cloud, use the the following command:

```
npm run migrate -- --hosts [NODE-IP] --username [USERNAME] --password[PASSWORD]
```

Replace the NODE-IP, USERNAME and PASSWORD with the information provided in your cluster on https://cloud.scylladb.com.

#### migrate/index.js

The above commands execute the main function in the `cmd/migrate/index.js`. The function creates the keyspace and tables that you need to run the collar and server services.

The below code in the `cmd/migrate/index.js` creates a new session then calls the `create_keyspace` , `migrate` functions.

```
migrate/index.js

async function main() {
  // Parse the command arguments: --hosts --username and --passowrd
  const options = config('migrate').parse().opts();
  
  // Create a new session with options 
  const client = await getClient(options);

  // Create a keyspace
  await client.execute(cql.KEYSPACE);
  
  // Create the tables
  for (const query of cql.MIGRATE) {
    log.debug(`query = ${query}`);
    await client.execute(query);
  }

  return client;
}
```
Let's breakdown the code above:

The `getClient` function takes the options as a parameter and creates a new session.

```
// src/db.js

async function getClient(config, keyspace) {
  const client = new cassandra.Client({
    contactPoints: config.hosts,
    authProvider: new cassandra.auth.PlainTextAuthProvider(
      config.username,
      config.password
    ),
    localDataCenter: 'datacenter1',
    keyspace,
  });

  await client.connect();

  return client;
}
```

`await client.execute(cql.KEYSPACE);` creates a keyspace as defined in `cql/keyspace.cql`:

```
CREATE KEYSPACE IF NOT EXISTS carepet WITH replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': '3' };
```

The CQL query above creates a new keyspace named carepet, with `NetworkTopologyStrategy` as replication strategy and a replication factor of 3.
More information about keyspace and replication on [Scylla University](https://university.scylladb.com/courses/data-modeling/lessons/basic-data-modeling-2/topic/keyspace/).

Finally, the code loops through all the queries listed in `cql/migrate.cql` to create the tables you need for the project.

```
CREATE TABLE IF NOT EXISTS carepet.owner
(
    owner_id UUID,
    address TEXT,
    name    TEXT,
    PRIMARY KEY (owner_id)
);

...
```

You can check the database structure. Connect to your local ScyllaDB instance using:
`docker exec -it carepet-scylla1 cqlsh`

with Scylla Cloud, use: 
```
docker run -it --rm --entrypoint cqlsh scylladb/scylla -u [USERNAME] -p [PASSOWRD] [NODE-IP]
```

Once connected to your machine, run the following commands:

```
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

The sensor service simulates the collar's activity and periodically saves data to the database. Use the below commands to run the sensor service:

#### Using ScyllaDB on your local machine
```
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
npm run sensor -- --hosts $NODE1 --measure 5s --buffer-interval 1m
```

#### Using Scylla Cloud
```
npm run sensor -- --hosts [NODE-IP] --username [USERNAME] --password [PASSOWRD] --measure 5s --buffer-interval 1m
```
Replace the NODE-IP, USERNAME and PASSWORD with the information provided in your cluster on https://cloud.scylladb.com.

The above command executes `cmd/sensor/index.js` and and takes the following as arguments.

-   `hosts` : the IP address of the ScyllaDB node.
-   `username`: when Password Authentication enabled
-   `passowrd`: when Password Authentication enabled
-   `measure`: the interval between to sensor measures.
-   `buffer-interval`: the interval between two database queries.

```
// sensor/index.js

async function main() {
  // Parse command arguments
  const options = cli(config('sensor simulator'))
    .parse()
    .opts();

  const bufferInterval = parseDuration(options.bufferInterval);
  const measure = parseDuration(options.measure);

  // ...
  
  // Connect to cluster using a keyspace
  const client = await getClientWithKeyspace(options);

  // Generate random owner, pet and sensors IDs
  const { owner, pet, sensors } = randomData();

  await saveData(client, owner, pet, sensors);

  // Generate sensor data and save them to the database periodically
  await runSensorData(
    client,
    {
      bufferInterval,
      measure,
    },
    sensors
  );

  return client;
}
```

Just like in `migrate/index.js`, the function parses the `npm run sensor` command arguments.
We then create a new session `client` using `carepet` keyspace.

```
// db.js

async function getClientWithKeyspace(config) {
  return getClient(config, KEYSPACE);
}
```

The `saveData` method connects to the datbase and saves random `owner`, `pet` and the `sensors` to the database.

```
// sensor/index.js

async function saveData(client, owner, pet, sensors) {
  await client.execute(insertQuery(Owner), owner, { prepare: true });
  log.info(`New owner # ${owner.owner_id}`);

  await client.execute(insertQuery(Pet), pet, { prepare: true });
  log.info(`New pet # ${pet.pet_id}`);

  for (let sensor of sensors) {
    await client.execute(insertQuery(Sensor), sensor, { prepare: true });
    log.info(`New sensor # ${sensor.sensor_id}`);
  }
}
```

The `runSensorData` generates random data and inserts it to the database every `buffer_interval`. Note that we are inserting the data to the database using a `batch`.

```
async function runSensorData(client, { bufferInterval, measure }, sensors) {
  let last = moment();
  while (true) {
    const measures = [];
    while (moment().diff(last) < bufferInterval) {
      await delay(measure);

      measures.push(
        ...sensors.map(sensor => {
          const measure = readSensorData(sensor);
          log.info(
            `sensor # ${sensor.sensor_id} type ${sensor.type} new measure ${
              measure.value
            } ts ${moment(measure.ts).toISOString()}`
          );

          return measure;
        })
      );
    }

    last = last.add(
      measure.valueOf() * (moment().diff(last).valueOf() / measure.valueOf())
    );

    log.info('Pushing data');

    const batch = measures.map(measure => ({
      query: insertQuery(Measure),
      params: measure,
    }));

    await client.batch(batch, { prepare: true });
  }
}
```

### Server

The server service is a REST API for tracking the pets’ health state. The service allows users to query the database via HTTP.

Run the following commands to start the server:

#### Using ScyllaDB on your local machine
```
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
npm run dev -- --hosts $NODE1
```

#### Using Scylla Cloud
```
npm run dev -- --hosts [NODE-IP] --username [USERNAME] --password [PASSOWRD]
```

The `src/index.js` main function mounts the api on `/api` and defines the routes.

```
// src/index.js

async function main() {
  const options = config('care-pet').parse().opts();

  log.debug(`Configuration = ${JSON.stringify(options)}`);

  const client = await getClientWithKeyspace(options);

  app.get(owner.ROUTE, asyncHandler(owner.handler(client)));
  app.get(pets.ROUTE, asyncHandler(pets.handler(client)));
  app.get(sensors.ROUTE, asyncHandler(sensors.handler(client)));
  app.get(measures.ROUTE, asyncHandler(measures.handler(client)));
  app.get(avg.ROUTE, asyncHandler(avg.handler(client)));

  app.listen(8000, () => {
    log.info('Care-pet server started on port 8000!');
  });
}
```

To test out the API in your terminal, use the following command: `$ curl http://127.0.0.1:8000/api/owner/{id}` and expect the following response:

````

[{"address":"home","age":57,"name":"tlmodylu","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360","pet_id":"a52adc4e-7cf4-47ca-b561-3ceec9382917","weight":5}]

```
