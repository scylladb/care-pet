Build an IoT App with JavaScript
------------------------

### Architecture

In this section, we will walk you through the CarePet commands and explain the code behind them.
The project is structured as follows:

![Build your first ScyllaDB Powered App - Raouf](https://user-images.githubusercontent.com/13738772/158383650-0dfcc9d0-68b5-457a-a043-27f6cda12de3.jpg)

- migrate (`npm run migrate`) - Creates the `carepet` keyspace and tables.
- collar (`npm run sensor`) - Generates pet health data and pushes it into the storage.
- web app (`npm run dev`) - REST API service for tracking pets' health state.

### Code Structure and Implementation

The code package structure is as follows:

| Name         | Purpose                             |
| ------------ | ----------------------------------- |
| /            | web application backend             |
| /api         | API spec                            |
| /cmd         | applications executables            |
| /cmd/migrate | install database schema             |
| /cmd/sensor  | Simulates the pet's collar          |
| /config      | database configuration              |
| /db          | database handlers (gocql/x)         |
| /db/cql      | database schema                     |
| /handler     | REST API handlers                   |
| /model       | application models and ORM metadata |

### Quick Start

Prerequisites:

- [NodeJS](https://nodejs.org/en/) tested with v17.0.1
- [NPM](https://www.npmjs.com/) tested with v8.1.0
- [docker](https://www.docker.com/) (not required if you use Scylla Cloud)
- [docker-compose](https://docs.docker.com/compose/) (not required if you use Scylla Cloud)


Clone the repository and change to `javascript` directory:
```
git clone git@github.com:scylladb/care-pet.git
cd javascript
```

Make sure to install all NodeJS dependencies with:

    $ npm install
    
### Use ScyllaDB on your local machine

To run a local ScyllaDB cluster consisting of three nodes with
the help of `docker` and `docker-compose` execute:

    $ docker-compose up -d

Docker-compose will spin up three nodes: `carepet-scylla1`, `carepet-scylla2`,
and `carepet-scylla3`. You can access them with the `docker` command.

Execute the following nodetool command:

    $ docker exec -it carepet-scylla1 nodetool status

### Migrate

#### Run ScyllaDB on your local machine

Once all the nodes are in UN - Up Normal status, run the commands below.

The following command allows you to get the node IP address:

```
docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1
```

The following commands execute the migrate `main` function.

```
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
npm run migrate -- --hosts $NODE1
```
You can check the database structure with:

```
docker exec -it carepet-scylla1 cqlsh
```

#### Using Scylla Cloud

If you are using Scylla Cloud, use the the following command to run the `migrate` service:

```
npm run migrate -- --hosts [NODE-IP] --username [USERNAME] --password[PASSWORD]
```

Replace the NODE-IP, USERNAME, and PASSWORD with the information provided in your cluster on https://cloud.scylladb.com.

### Output

Expected output:
```
2020/08/06 16:43:01 Bootstrap database...
2020/08/06 16:43:13 Keyspace metadata = {Name:carepet DurableWrites:true StrategyClass:org.apache.cassandra.locator.NetworkTopologyStrategy StrategyOptions:map[datacenter1:3] Tables:map[gocqlx_migrate:0xc00016ca80 measurement:0xc00016cbb0 owner:0xc00016cce0 pet:0xc00016ce10 sensor:0xc00016cf40 sensor_avg:0xc00016d070] Functions:map[] Aggregates:map[] Types:map[] Indexes:map[] Views:map[]}
```

You can check the database structure with:

`docker run -it --rm --entrypoint cqlsh scylladb/scylla -u [USERNAME] -p [PASSWORD] [NODE-IP]`

Note: use `-u [USERNAME]` and `-p [PASSWORD]` if you are using Scylla Cloud.

Once you connect to cqlsh, run the following commands:

#. Run `DESCRIBE KEYSPACES`.

Expected output: 
```
carepet  system_schema  system_auth  system  system_distributed  system_traces
```
then, 
```
 carepet;
DESCRIBE TABLES
```
Expected output: 

`pet  sensor_avg  gocqlx_migrate  measurement  owner  sensor`

#. Run `DESCRIBE TABLE pet`.

Expected output:
```
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
       AND dclocal_read_repair_chance = 0.1
       AND default_time_to_live = 0
       AND gc_grace_seconds = 864000
       AND max_index_interval = 2048
       AND memtable_flush_period_in_ms = 0
       AND min_index_interval = 128
       AND read_repair_chance = 0.0
       AND speculative_retry = '99.0PERCENTILE';
```

#. Run `exit` to exit the cqlsh.

#### migrate/index.js

The above commands execute the main function in the `cmd/migrate/index.js`. The function creates the keyspace and tables that you need to run the collar and server services.

The following code in the `cmd/migrate/index.js` creates a new session, then calls the `create_keyspace`  and `migrate` functions.

```
migrate/index.js

async function main() {
  // Parse the command arguments: --hosts --username and --password
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
Let's break down the code above.

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
See [Scylla University](https://university.scylladb.com/courses/data-modeling/lessons/basic-data-modeling-2/topic/keyspace/) for more information about keyspaces and replication.

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

With Scylla Cloud, use: 
```
docker run -it --rm --entrypoint cqlsh scylladb/scylla -u [USERNAME] -p [PASSWORD] [NODE-IP]
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
npm run sensor -- --hosts [NODE-IP] --username [USERNAME] --password [PASSWORD] --measure 5s --buffer-interval 1m
```
Replace the NODE-IP, USERNAME, and PASSWORD with the information provided in your cluster on https://cloud.scylladb.com.

Expected output:
```
2020/08/06 16:44:33 Welcome to the Pet collar simulator
2020/08/06 16:44:33 New owner # 9b20764b-f947-45bb-a020-bf6d02cc2224
2020/08/06 16:44:33 New pet # f3a836c7-ec64-44c3-b66f-0abe9ad2befd
2020/08/06 16:44:33 sensor # 48212af8-afff-43ea-9240-c0e5458d82c1 type L new measure 51.360596 ts 2020-08-06T16:44:33+02:00
2020/08/06 16:44:33 sensor # 2ff06ffb-ecad-4c55-be78-0a3d413231d9 type R new measure 36 ts 2020-08-06T16:44:33+02:00
2020/08/06 16:44:33 sensor # 821588e0-840d-48c6-b9c9-7d1045e0f38c type L new measure 26.380281 ts 2020-08-06T16:44:33+02:00
...
```

The above command executes `cmd/sensor/index.js` and takes the following as arguments:

-   `hosts` : the IP address of the ScyllaDB node.
-   `username`: when Password Authentication enabled
-   `password`: when Password Authentication enabled
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
Now you can create a new session `client` using `carepet` keyspace.

```
// db.js

async function getClientWithKeyspace(config) {
  return getClient(config, KEYSPACE);
}
```

The `saveData` method connects to the database and saves random `owner`, `pet`, and `sensors` to the database.

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

The server service is a REST API for tracking the pets’ health state. The service allows you to query the database via HTTP.

Run the following commands to start the server:

#### Using ScyllaDB on your local machine
```
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
npm run dev -- --hosts $NODE1
```

#### Using Scylla Cloud
```
npm run dev -- --hosts [NODE-IP] --username [USERNAME] --password [PASSWORD]
```

Expected output:

```
2020/08/06 16:45:58 Serving care pet at http://127.0.0.1:8000
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

### Using the Application 

Open a different terminal to send an HTTP request from the CLI:

`curl -v http://127.0.0.1:8000/`

Expected output:

    > GET / HTTP/1.1
    > Host: 127.0.0.1:8000
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
    * Closing connection 0
    {"code":404,"message":"path / was not found"}

The JSON with the 404 at the end indicates expected behavior.
To read an owner's data use the previously saved owner_id as follows:

`curl -v http://127.0.0.1:8000/api/owner/{owner_id}`

For example:

`curl http://127.0.0.1:8000/api/owner/a05fd0df-0f97-4eec-a211-cad28a6e5360`

Expected result:

    {"address":"home","name":"gmwjgsap","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360"} 

To list the owner's pets, run:

`curl -v http://127.0.0.1:8000/api/owner/{owner_id}/pets`

For example:

`curl http://127.0.0.1:8000/api/owner/a05fd0df-0f97-4eec-a211-cad28a6e5360/pets`

Expected output:

`[{"address":"home","age":57,"name":"tlmodylu","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360","pet_id":"a52adc4e-7cf4-47ca-b561-3ceec9382917","weight":5}]`

To list each pet's sensor, run:

`curl -v curl -v http://127.0.0.1:8000/api/pet/{pet_id}/sensors`

For example:

`curl http://127.0.0.1:8000/api/pet/cef72f58-fc78-4cae-92ae-fb3c3eed35c4/sensors`

```
[{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d","type":"L"},{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"5c70cd8a-d9a6-416f-afd6-c99f90578d99","type":"R"},{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"fbefa67a-ceb1-4dcc-bbf1-c90d71176857","type":"L"}]
```

To review the data from a specific sensor:

`curl http://127.0.0.1:8000/api/sensor/{sensor_id}/values?from=2006-01-02T15:04:05Z07:00&to=2006-01-02T15:04:05Z07:00`

For example:

`curl http://127.0.0.1:8000/api/sensor/5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d/values\?from\="2020-08-06T00:00:00Z"\&to\="2020-08-06T23:59:59Z"`

expected output:

`[51.360596,26.737432,77.88015,...]`

To read the pet's daily average per sensor, use:

`curl http://127.0.0.1:8000/api/sensor/{sensor_id}/values/day/{date}`

For example:

`curl -v http://127.0.0.1:8000/api/sensor/5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d/values/day/2020-08-06`

Expected output:

`[0,0,0,0,0,0,0,0,0,0,0,0,0,0,42.55736]`
