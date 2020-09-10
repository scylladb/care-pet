Care Pet ScyllaDB IoT example
===

This example project demonstrates a generic IoT use case
for ScyllaDB in Java.
The documentation for this application and guided exercise is [here](docs).

The application allows tracking of pets health indicators
and consist of three parts:

- migrate (`com.carepet.Migrate`) - creates the `carepet` keyspace and tables
- collar (`com.carepet.Sensor`) - generates a pet health data and pushes it into the storage
- web app (`com.carepet.server.App`) - REST API service for tracking pets health state

Quick Start
---

Prerequisites:

- [JDK](https://openjdk.java.net/install/) at least OpenJDK 8
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

    SLF4J: Class path contains multiple SLF4J bindings.
    SLF4J: Found binding in [jar:file:/home/sitano/.m2/repository/org/slf4j/slf4j-simple/1.7.26/slf4j-simple-1.7.26.jar!/org/slf4j/impl/StaticLoggerBinder.class]
    SLF4J: Found binding in [jar:file:/home/sitano/.m2/repository/ch/qos/logback/logback-classic/1.2.3/logback-classic-1.2.3.jar!/org/slf4j/impl/StaticLoggerBinder.class]
    SLF4J: See http://www.slf4j.org/codes.html#multiple_bindings for an explanation.
    SLF4J: Actual binding is of type [org.slf4j.impl.SimpleLoggerFactory]
    [main] INFO com.carepet.Migrate - creating keyspace...
    Using Scylla optimized driver!!!
    [main] INFO com.datastax.oss.driver.api.core.session.SessionBuilder - Using Scylla optimized driver!!!
    [main] INFO com.datastax.oss.driver.internal.core.DefaultMavenCoordinates - DataStax Java driver for Apache Cassandra(R) (com.scylladb:java-driver-core) version 4.8.0-scylla-0
    [s0-admin-0] INFO com.datastax.oss.driver.internal.core.time.Clock - Using native clock for microsecond precision
    [main] INFO com.carepet.Migrate - creating table...
    Using Scylla optimized driver!!!
    [main] INFO com.datastax.oss.driver.api.core.session.SessionBuilder - Using Scylla optimized driver!!!
    [s1-admin-0] INFO com.datastax.oss.driver.internal.core.time.Clock - Using native clock for microsecond precision
    Using Scylla optimized driver!!!
    [main] INFO com.datastax.oss.driver.api.core.session.SessionBuilder - Using Scylla optimized driver!!!
    [s2-admin-0] INFO com.datastax.oss.driver.internal.core.time.Clock - Using native clock for microsecond precision
    Keyspace: carepet; Table: measurement
    Keyspace: carepet; Table: owner
    Keyspace: carepet; Table: pet
    Keyspace: carepet; Table: sensor
    Keyspace: carepet; Table: sensor_avg

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

    cqlsh:carepet> exit

...

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

How to gather all the dependencies to run the app
---

You have a few options here:

- pack all of them into a single jar
- copy all of them into the build folder and include into classpath

Let's take a look at path 2:

    <plugins>
      <plugin>
        <groupId>org.apache.maven.plugins</groupId>
        <artifactId>maven-dependency-plugin</artifactId>
        <version>3.1.2</version>
        <executions>
          <!-- copy dependencies -->
          <execution>
            <id>copy-dependencies</id>
            <phase>generate-sources</phase>
            <goals>
              <goal>copy-dependencies</goal>
            </goals>
            <configuration>
              <overWriteReleases>false</overWriteReleases>
              <overWriteSnapshots>false</overWriteSnapshots>
              <overWriteIfNewer>true</overWriteIfNewer>
            </configuration>
          </execution>
          <!-- build class path -->
          <execution>
            <id>build-classpath</id>
            <phase>generate-sources</phase>
            <goals>
              <goal>build-classpath</goal>
            </goals>
            <configuration>
              <outputFile>${project.build.directory}/dependencies</outputFile>
            </configuration>
          </execution>
        </executions>
      </plugin>
    </plugins>
    
This will copy all the dependencies to the `target/dependency` folder and create
a `dependencies` file that can be included into the `-classpath` definition:

    $ mvn package
    $ java -classpath ./target/sample-1.0-SNAPSHOT.jar:$(cat ./target/dependencies) com.project.App

Links
---

- https://hub.docker.com/r/scylladb/scylla/
- https://github.com/scylladb/java-driver/tree/4.x/

