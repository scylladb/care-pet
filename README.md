ScyllaDB IoT example
===

This example project demonstrates a generic IoT use case
for ScyllaDB.

The application allows tracking of pets' health indicators
and consists of 3 parts:

- a collar that reads and pushes sensors data
- a web app for reading and analyzing the pets' data
- a database migration tool

Read the [Documentation for the Care-Pet Example](https://iot.scylladb.com)

Quick Start
---

Language-specific implementation resides in the corresponding folder:

- [Go](go)
- [Java](java)
- [Javascript/NodeJS](javascript)
- [PHP](php)
- [Rust](rust)
- CPP
- Python
- [CSharp](csharp)


The project uses `docker-compose` to provide the simplest local
deployment of the ScyllaDB database cluster.

Architecture
---

```
Pet -> Sensors -> Collar -> ScyllaDB <---- WebApp <- User
                               ^             |
                               \-aggregation-/
```

In this simple example, a Collar generates sensors data
once a second and sends it directly to the database.

Pet owners, sensors, and measurement data can be accessed via
the REST API web server. It also implements the lazy-evaluation
of the data aggregates.

Links
---
- [Care-Pet Example Guide](https://care-pet.docs.scylladb.com/)
- [Scylla Docs](https://docs.scylladb.com/)
- [ScyllaDB Docker image](https://hub.docker.com/r/scylladb/scylla/)
- [ScyllaDB drivers page](https://docs.scylladb.com/using-scylla/scylla_drivers/)
