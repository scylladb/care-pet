Care Pet ScyllaDB IoT example
===

This example project demonstrates a generic IoT use case
for ScyllaDB in Java.
The documentation for this application and guided excercise is [here](docs).

The application allows tracking of pets health indicators
and consist of three parts:

- migrate (`com.carepet.Migrate`) - creates the `carepet` keyspace and tables
- collar (`com.carepet.Sensor`) - generates a pet health data and pushes it into the storage
- web app (`com.carepet.Server`) - REST API service for tracking pets health state

Quick Start
---

Prerequisites:

- [go](https://golang.org/dl/) at least 1.14
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

You can inspect any node by means of the `docker inspect` command
as follows. for example:

    $ docker inspect carepet-scylla1

To get node IP address run:

    $ docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1

To initialize database execute:

    $ mvn package
    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./bin/migrate.sh --hosts $NODE1 --datacenter datacenter1

Expected output:

    2020/08/06 16:43:01 Bootstrap database...
    2020/08/06 16:43:13 Keyspace metadata = {Name:carepet DurableWrites:true StrategyClass:org.apache.cassandra.locator.NetworkTopologyStrategy StrategyOptions:map[datacenter1:3] Tables:map[gocqlx_migrate:0xc00016ca80 measurement:0xc00016cbb0 owner:0xc00016cce0 pet:0xc00016ce10 sensor:0xc00016cf40 sensor_avg:0xc00016d070] Functions:map[] Aggregates:map[] Types:map[] Indexes:map[] Views:map[]}
    
    Using Scylla optimized driver!!!
    [main] INFO com.datastax.oss.driver.api.core.session.SessionBuilder - Using Scylla optimized driver!!!
    [main] INFO com.datastax.oss.driver.internal.core.DefaultMavenCoordinates - DataStax Java driver for Apache Cassandra(R) (com.scylladb:java-driver-core) version 4.8.0-scylla-0
    [s0-admin-0] INFO com.datastax.oss.driver.internal.core.time.Clock - Using native clock for microsecond precision
    Using Scylla optimized driver!!!
    [main] INFO com.datastax.oss.driver.api.core.session.SessionBuilder - Using Scylla optimized driver!!!
    [s1-admin-0] INFO com.datastax.oss.driver.internal.core.time.Clock - Using native clock for microsecond precision

You can check the database structure with:

    $ docker exec -it carepet-scylla1 cqlsh
    cqlsh> DESCRIBE KEYSPACES

    carepet  system_schema  system_auth  system  system_distributed  system_traces

    cqlsh> USE carepet;
    cqlsh:carepet> DESCRIBE TABLES

    pet  sensor_avg  gocqlx_migrate  measurement  owner  sensor

    cqlsh:carepet> DESCRIBE TABLE pet

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
        AND dclocal_read_repair_chance = 0.1
        AND default_time_to_live = 0
        AND gc_grace_seconds = 864000
        AND max_index_interval = 2048
        AND memtable_flush_period_in_ms = 0
        AND min_index_interval = 128
        AND read_repair_chance = 0.0
        AND speculative_retry = '99.0PERCENTILE';

    cqlsh:carepet> exit

To start pet collar simulation execute the following in the separate terminal:

    $ go build ./cmd/sensor
    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./sensor --hosts $NODE1

Expected output:

    2020/08/06 16:44:33 Welcome to the Pet collar simulator
    2020/08/06 16:44:33 New owner # 9b20764b-f947-45bb-a020-bf6d02cc2224
    2020/08/06 16:44:33 New pet # f3a836c7-ec64-44c3-b66f-0abe9ad2befd
    2020/08/06 16:44:33 sensor # 48212af8-afff-43ea-9240-c0e5458d82c1 type L new measure 51.360596 ts 2020-08-06T16:44:33+02:00
    2020/08/06 16:44:33 sensor # 2ff06ffb-ecad-4c55-be78-0a3d413231d9 type R new measure 36 ts 2020-08-06T16:44:33+02:00
    2020/08/06 16:44:33 sensor # 821588e0-840d-48c6-b9c9-7d1045e0f38c type L new measure 26.380281 ts 2020-08-06T16:44:33+02:00
    ...

Write down the pet Owner ID (ID is something after the `#` sign without trailing spaces).
To start REST API service execute the following in the separate terminal:

    $ go build ./cmd/server
    $ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
    $ ./server --port 8000 --hosts $NODE1

Expected output:

    2020/08/06 16:45:58 Serving care pet at http://127.0.0.1:8000

Now you can open `http://127.0.0.1:8000/` in the browser or send an HTTP request from the CLI:

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

This is ok. If you see this JSON in the end with 404, it means everything works as expected.
To read an owner data you can use saved `owner_id` as follows:

    $ curl -v http://127.0.0.1:8000/api/owner/{owner_id}

For example:

    $ curl http://127.0.0.1:8000/api/owner/a05fd0df-0f97-4eec-a211-cad28a6e5360

Expected result:

    {"address":"home","name":"gmwjgsap","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360"}

To list the owners pets use:

    $ curl -v http://127.0.0.1:8000/api/owner/{owner_id}/pets

For example:

    $ curl http://127.0.0.1:8000/api/owner/a05fd0df-0f97-4eec-a211-cad28a6e5360/pets

Expected output:

    [{"address":"home","age":57,"name":"tlmodylu","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360","pet_id":"a52adc4e-7cf4-47ca-b561-3ceec9382917","weight":5}]

To list pet's sensors use:

    $ curl -v curl -v http://127.0.0.1:8000/api/pet/{pet_id}/sensors

For example:

    $ curl http://127.0.0.1:8000/api/pet/cef72f58-fc78-4cae-92ae-fb3c3eed35c4/sensors

    [{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d","type":"L"},{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"5c70cd8a-d9a6-416f-afd6-c99f90578d99","type":"R"},{"pet_id":"cef72f58-fc78-4cae-92ae-fb3c3eed35c4","sensor_id":"fbefa67a-ceb1-4dcc-bbf1-c90d71176857","type":"L"}]

To review the pet's sensors data use:

    $ curl http://127.0.0.1:8000/api/sensor/{sensor_id}/values?from=2006-01-02T15:04:05Z07:00&to=2006-01-02T15:04:05Z07:00

For example:

    $  curl http://127.0.0.1:8000/api/sensor/5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d/values\?from\="2020-08-06T00:00:00Z"\&to\="2020-08-06T23:59:59Z"

 Expected output:

    [51.360596,26.737432,77.88015,...]

To read the pet's daily average per sensor use:

    $ curl http://127.0.0.1:8000/api/sensor/{sensor_id}/values/day/{date}

For example:

    $ curl -v http://127.0.0.1:8000/api/sensor/5a9da084-ea49-4ab1-b2f8-d3e3d9715e7d/values/day/2020-08-06

Expected output:

    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,42.55736]

Structure
---

Package structure is as follows:

| Name         | Purpose                                   |
| ----         | -------                                   |
| /api         | swagger api spec                          |
| /cmd         | applications executables                  |
| /cmd/migrate | install database schema                   |
| /cmd/sensor  | simulate pet collar                       |
| /cmd/server  | web application backend                   |
| /config      | database configuration                    |
| /db          | database handlers (gocql/x)               |
| /db/cql      | database schema                           |
| /handler     | swagger REST API handlers                 |
| /model       | application models and ORM metadata       |

API
---

Swagger [specification](api/api.json).

Implementation
---

Collars are small devices that attach to pets and collect data
with the help of different sensors. After the data is collected it
may be delivered to the central database for the analysis and
health status checking.

Collar code sits in the `/cmd/sensor` and uses `scylladb/gocqlx`
Go driver to connect to the database directly and publish its data.
Collar sends sensor measurements updates every once in a second.

Overall all applications in this repository use `scylladb/gocqlx` for:

- Relational Object Mapping (ORM)
- Build Queries
- Migrate database schemas

The web application REST API server resides in `/cmd/server` and uses
`go-swagger` that supports OpenAPI 2.0 to expose its API. API
handlers reside in `/handler`. Most of the queries are reads.

The application is capable of caching sensor measurements data
on hourly basis. It uses lazy evaluation to manage `sensor_avg`.
It can be viewed as an application-level lazy-evaluated
materialized view.

The algorithm is simple and resides in `/handler/avg.go`:

- read `sensor_avg`
- if no data, read `measurement` data, aggregate in memory, save
- serve request

Architecture
---

    Pet --> Sensor --> ScyllaDB <-> REST API Server <-> User

How to start a new project with Java
---

Install JDK >= 8 and Maven. Create a repository. Clone it. Execute inside of
your repository:

    $ mvn archetype:generate -DgroupId=com.mycompany.app -DartifactId=my-app -DarchetypeArtifactId=maven-archetype-quickstart -DarchetypeVersion=1.4 -DinteractiveMode=false

Now when you have your pom module add ScyllaDB driver as a dependency with:

    <dependencies>
        <dependency>
          <groupId>com.scylladb</groupId>
          <artifactId>java-driver-core</artifactId>
          <version>4.8.0-scylla-0</version>
        </dependency>
    </dependencies>

Now your `pom.xml` will be looking something like this:

    <?xml version="1.0" encoding="UTF-8"?>

    <project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
      xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
      <modelVersion>4.0.0</modelVersion>

      <groupId>com.mycompany.app</groupId>
      <artifactId>my-app</artifactId>
      <version>1.0-SNAPSHOT</version>

      <name>my-app</name>
      <!-- FIXME change it to the project's website -->
      <url>http://www.example.com</url>

      <properties>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
        <maven.compiler.source>1.7</maven.compiler.source>
        <maven.compiler.target>1.7</maven.compiler.target>
      </properties>

      <dependencies>
          <dependency>
            <groupId>com.scylladb</groupId>
            <artifactId>java-driver-core</artifactId>
            <version>4.8.0-scylla-0</version>
          </dependency>
          ...
      </dependencies>

      <build>
        <pluginManagement><!-- lock down plugins versions to avoid using Maven defaults (may be moved to parent pom) -->
            ...
        </pluginManagement>
      </build>
    </project>
   
Now you are ready to connect to the database and start working.
To connect to the database, do the following:

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.CqlSessionBuilder;

class Example {
    public static void main(String []args) {
        CqlSessionBuilder builder = CqlSession.builder()
                .withApplicationName(applicationName)
                .withClientId(clientId);

        CqlSession session = builder.build();
    }
}
```

If you want to use authentication it can be done with:

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.CqlSessionBuilder;

class Example {
    public static void main(String []args) {
        CqlSession session = CqlSession.builder()
                .withAuthCredentials("username", "password")
                .build();
    }
}
```

Local endpoints also require specifying local datacenter:

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.CqlSessionBuilder;
import java.net.InetSocketAddress;

class Example {
    public static void main(String []args) {
        CqlSession session = CqlSession.builder()
                .addContactPoints({new InetSocketAddress("127.0.0.1", 9042)})
                .withLocalDatacenter("datacenter1")
                .build();
    }
}
```

Now you can issue CQL commands with:

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.CqlSessionBuilder;
import com.datastax.oss.driver.api.core.cql.PreparedStatement;import com.datastax.oss.driver.api.core.cql.ResultSet;
import com.datastax.oss.driver.api.core.cql.Row;

class Example {
    public static void main(String []args) {
        CqlSession session = CqlSession.builder().build();

        session.execute("INSERT INTO table VALUE(1, 2, 3)");

        // or
        PreparedStatement statement = session.prepare("INSERT INTO table VALUE(?, ?, ?)");
        session.execute(statement.bind(1, 2, 3));

        // or
        ResultSet s = session.execute("SELECT * FROM table");
        for (Row r: s) {
            // r.get()
        }
    }
}
```

You can use query builder with the help of:

    <dependency>
      <groupId>com.scylladb</groupId>
      <artifactId>java-driver-query-builder</artifactId>
      <version>4.8.0-scylla-0</version>
    </dependency>

To get:

    Statement stmt =
        selectFrom("examples", "querybuilder_json")
            .json()
            .all()
            .whereColumn("id")
            .isEqualTo(literal(1))
            .build();
            
To use object-data mapping (ORM) include:

    <dependency>
      <groupId>com.scylladb</groupId>
      <artifactId>java-driver-mapper-runtime</artifactId>
      <version>4.8.0-scylla-0</version>
    </dependency>
    
Add annotation processing:

    <build>
      <plugins>
        <plugin>
          <artifactId>maven-compiler-plugin</artifactId>
          <version>3.8.0</version>
          <configuration>
            <annotationProcessorPaths>
              <path>
                <groupId>com.scylladb</groupId>
                <artifactId>java-driver-mapper-processor</artifactId>
                <version>4.8.0-scylla-0</version>
              </path>
            </annotationProcessorPaths>
            <compilerArgs>
              <arg>-Aproject=${project.groupId}/${project.artifactId}</arg>
            </compilerArgs>
          </configuration>
        </plugin>
      </plugins>
    </build>
    
Create a mapper:

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.mapper.annotations.DaoFactory;

@com.datastax.oss.driver.api.mapper.annotations.Mapper
public interface Mapper {
    static com.datastax.oss.driver.api.mapper.MapperBuilder<Mapper> builder(CqlSession session) {
        return new MapperBuilder(session);
    }
}
```

You can create DAO factory per item type or one for all types:

```java
import com.datastax.oss.driver.api.mapper.annotations.Dao;
import com.datastax.oss.driver.api.mapper.annotations.Select;

import java.util.UUID;

@Dao
public interface DAO {
        /** Simple selection by full primary key. */
        @Select
        Owner getOwner(UUID id);
}
```

Generate the source with:

    $ mvn compile

Add DAO factory:

```java
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.mapper.annotations.DaoFactory;

@com.datastax.oss.driver.api.mapper.annotations.Mapper
public interface Mapper {
    @DaoFactory
    DAO dao();

    static com.datastax.oss.driver.api.mapper.MapperBuilder<Mapper> builder(CqlSession session) {
        return new MapperBuilder(session);
    }
}
```

You are ready to go.

For more details, check out implementation.

Links
---

- https://hub.docker.com/r/scylladb/scylla/
- https://github.com/scylladb/java-driver/tree/4.x/

