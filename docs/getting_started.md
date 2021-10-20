Getting Started with CarePet: A sample IoT App
----------------------------------------------
## Table of Contents

* [Introduction](#introduction)
* [Requirements](#requirements)
* [Deploying the Application](#deploying-the-application)
* [Using the Application ](#using-the-application )
* [Code Structure and Implementation](#code-structure-and-implementation)
* [Design and Data Model](#design-and-data-model)
* [Additional Resources](#additional-resources)


## Introduction

This guide will show you how to create an IoT app from scratch and configure it
to use Scylla as the backend datastore. It'll walk you through all the stages
of the development process, from gathering requirements to building and running
the application.

As an example, you will us an application called CarePet. CarePet allows pet
owners to track their pets' health by monitoring their key health parameters,
such as temperature or pulse. The application consists of three parts:

-   A pet collar with sensors that collects pet health data and sends the data to the datastore.
-   A web app for reading the data and analyzing the pets' health.
-   A database migration module.

You can use this example with minimal changes for any IoT application.

## Requirements

### Prerequisites for Deploying the Application

-   [Go](https://golang.org/dl/) 1.14 or later
-   [Docker](https://www.docker.com/)
-   [Docker Compose](https://docs.docker.com/compose/)

### Use Case Requirements

Each pet collar has sensors that report four different measurements:
temperature, pulse, location, and respiration.

The collar reads the measurements from the sensors once per second
and sends the data directly to the app.

### Performance Requirements

The application has two performance-related parts: sensors that write to
the database (throughput sensitive) and backend dashboard that reads from
the database (latency sensitive). 
  
* This example assumes 99% writes (sensors) and 1% reads (backend dashboard).  
* Service Level Objectives (SLO):
  - Writes throughput of 100K operations per second
  - Reads: latency of up to 10 milliseconds for the
    [99th percentile](https://engineering.linkedin.com/performance/who-moved-my-99th-percentile-latency).
* The application requires high availability and fault tolerance. Even if a
ScyllaDB node goes down or becomes unavailable, the cluster is expected to
remain available and continue to provide service. You can learn more about
Scylla high availability in [this lesson](https://university.scylladb.com/courses/scylla-essentials-overview/lessons/high-availability/). 

## Deploying the Application

The example application uses Docker to run a three-node ScyllaDB cluster. It
consists of three parts:

-   migrate (/cmd/migrate) - Creates the CarePet keyspace and tables.
-   collar (/cmd/sensor) - Generates a pet health data and pushes it into the
    storage.
-   web app (/cmd/server) - REST API service for tracking the pets' health
    state.


1. Check out the care-pet repository from GitHub to get a copy of
the application code:

    `$ git clone git@github.com:scylladb/care-pet.git`

2. Create a local ScyllaDB cluster consisting of three nodes:

    `$ docker-compose up -d`

   Docker-compose will spin up a ScyllaDB cluster consisting of tree nodes:
   carepet-scylla1, carepet-scylla2 and carepet-scylla3. The process will
   take about two minutes.

3. Check the status of the cluster:

    `$ docker exec -it carepet-scylla1 nodetool status`

4. Once all the nodes are in UN - Up Normal status, initialize the database
to create keyspaces and tables:
	```
    $ go build ./cmd/migrate
    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./migrate --hosts $NODE1
	```

	Expected output:
    ```
    2020/08/06 16:43:01 Bootstrap database...
    2020/08/06 16:43:13 Keyspace metadata = {Name:carepet DurableWrites:true StrategyClass:org.apache.cassandra.locator.NetworkTopologyStrategy StrategyOptions:map[datacenter1:3] Tables:map[gocqlx_migrate:0xc00016ca80 measurement:0xc00016cbb0 owner:0xc00016cce0 pet:0xc00016ce10 sensor:0xc00016cf40 sensor_avg:0xc00016d070] Functions:map[] Aggregates:map[] Types:map[] Indexes:map[] Views:map[]}
	```

5. (Optional)You can check the database structure with the following command:
    ```
    $ docker exec -it carepet-scylla1 cqlsh
    
    cqlsh> DESCRIBE KEYSPACES
    carepet  system_schema  system_auth  system  system_distributed  system_traces
    
    cqlsh> USE carepet;
    cqlsh:carepet> DESCRIBE TABLES
    pet  sensor_avg  gocqlx_migrate  measurement  owner  sensor
    
    cqlsh:carepet> DESCRIBE TABLE pet
    
    CREATE TABLE carepet.pet (
         owner_id uuid,
         pet_id uuid,
         address text
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
    
    cqlsh:carepet> exit
    ```

6. Execute the following command from a separate terminal to start the pet
collar simulation:
    ```
    $ go build ./cmd/sensor
    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./sensor --hosts $NODE1
	```
	
	This will generate the pets' health data and save it to the database.

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

7. Make a note of the pet owner, pet, and sensor IDs for future reference. IDs are
the numbers after the # sign (without trailing spaces). The IDs will be useful
while using the application.
8. Start the REST API service in a separate terminal (this will be the third
terminal you are using for deployment). The server exposes a REST API that
allows tracking the pets' health parameters:
    ```
    $ go build ./cmd/server
    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./server --port 8000 --hosts $NODE1
	```

	Expected output:
	
    `2020/08/06 16:45:58 Serving care pet at http://127.0.0.1:8000`


## Using the Application 

### Accessing the Application

Open http://127.0.0.1:8000/ in a browser or send an HTTP request from the
command line interface to access the application:

    $ curl -v http://127.0.0.1:8000/

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


The JSON with the 404 error message at the end indicates expected behavior. 
	
### Displaying the Owner Data

To read an owner's data, use the owner ID ( see step 7 in [Deploying the Application](#deploying-the-application)):

    $ curl -v http://127.0.0.1:8000/api/owner/{owner_id}

Your command line may resemble the following:

    $ curl http://127.0.0.1:8000/api/owner/a05fd0df-0f97-4eec-a211-cad28a6e5360

Expected result:

{"address":"home","name":"gmwjgsap","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360"} 

### Listing Pets

To list the owner's pets, run:

    $ curl -v http://127.0.0.1:8000/api/owner/{owner_id}/pets

Your command line may resemble the following:

    $ curl http://127.0.0.1:8000/api/owner/a05fd0df-0f97-4eec-a211-cad28a6e5360/pets

Expected output:

    [{"address":"home","age":57,"name":"tlmodylu","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360","pet_id":"a52adc4e-7cf4-47ca-b561-3ceec9382917","weight":5}]

### Listing Sensors

To list all sensors of a given pet,  use the pet ID ( see step 7 in [Deploying the Application](#deploying-the-application)):

    $ curl -v curl -v http://127.0.0.1:8000/api/pet/{pet_id}/sensors

Your command line may resemble the following:

    $ curl http://127.0.0.1:8000/api/pet/cef72f58-fc78-4cae-92ae-fb3c3eed35c4/sensors

Expected output:

    [{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d","type":"L"},{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"5c70cd8a-d9a6-416f-afd6-c99f90578d99","type":"R"},{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"fbefa67a-ceb1-4dcc-bbf1-c90d71176857","type":"L"}]

### Displaying Data Collected by a Sensor

To review the data from a a sensor, use a sensor ID ( see step 7 in [Deploying the Application](#deploying-the-application)):

    $ curl http://127.0.0.1:8000/api/sensor/{sensor_id}/values?from=2006-01-02T15:04:05Z07:00&to=2006-01-02T15:04:05Z07:00

Your command line may resemble the following:

    $  curl http://127.0.0.1:8000/api/sensor/5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d/values\?from\="2020-08-06T00:00:00Z"\&to\="2020-08-06T23:59:59Z"

Expected output:

    [51.360596,26.737432,77.88015,...]

### Displaying the Avergage Value per Sensor

To read the daily average value for a given sensor, use the sensor ID:

    $ curl http://127.0.0.1:8000/api/sensor/{sensor_id}/values/day/{date}

Your command line may resemble the following:

    $ curl -v http://127.0.0.1:8000/api/sensor/5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d/values/day/2020-08-06

Expected output:

    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,42.55736]

## Code Structure and Implementation

The code package structure is as follows:

| Name         | Purpose                             |
| ------------ | ----------------------------------- |
| /api         | swagger api spec                    |
| /cmd         | applications executables            |
| /cmd/migrate | install database schema             |
| /cmd/sensor  | Simulates the pet's collar          |
| /cmd/server  | web application backend             |
| /config      | database configuration              |
| /db          | database handlers (gocql/x)         |
| /db/cql      | database schema                     |
| /handler     | swagger REST API handlers           |
| /model       | application models and ORM metadata |

After data is collected from the pets via the sensors on their collars, it is
delivered to the central database for analysis and for health status checking.

The collar code sits in the /cmd/sensor and uses scylladb/gocqlx Go driver to
connect to the database directly and publish its data. The collar sends a
sensor measurement update once a second.

Overall all applications in this repository use scylladb/gocqlx for:

-   Relational Object Mapping (ORM)
-   Building queries
-   Migrating database schemas

The web application's REST API server resides in /cmd/server and uses
go-swagger that supports OpenAPI 2.0 to expose its API. API handlers reside in
/handler. Most of the queries are reads.

The application is capable of caching sensor measurements data on an hourly
basis. It uses Lazy Evaluation to manage sensor_avg. It can be viewed as an
application-level lazy-evaluated materialized view. 

The algorithm is simple and resides in /handler/avg.go:

-   read sensor_avg
-   if there is no data, read measurement data, aggregate in memory, save
-   serve request

## Design and Data Model

See [Design and Data Model](./design_and_data_model.md) for details about data
modeling in Scylla.

## Additional Resources

-   [Scylla Essentials](https://university.scylladb.com/courses/scylla-essentials-overview/) course on Scylla University. It provides an introduction to Scylla and explains the basics.
-   [Data Modeling and Application Development](https://university.scylladb.com/courses/data-modeling/) course on Scylla University. It explains basic and advanced data modeling techniques, including information on workflow application, query analysis, denormalization, and other NoSQL data modeling topics.
-   [Scylla Documentation](https://docs.scylladb.com/)
-   [Slack channel](http://slack.scylladb.com/) for Scylla users

Future Work

-   Add Sizing
-   Add Benchmarking
-   Add Java implementation
-   Add Python implementation
-   In a real-world application, it would be better to aggregate data in an internal buffer and send it once a day to the application gateway in a batch, implying techniques such as delta encoding. It could also aggregate data at a lower resolution and take measurements less frequently. The collar could notify the pet's owner about suspicious health parameters directly or via the application. 
-   Add location tracking info to send alerts when the pet enters/leaves safe zones using known WiFi networks.
-   Use the measurements to present to the pet owner health alerts, vital signs, sleeping levels, activity levels, and calories burned.
