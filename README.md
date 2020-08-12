ScyllaDB IoT example
===

This is an example project that demonstrates a generic IoT use case
for ScyllaDB.

The application allows tracking of pets health indicators
and consist of 3 parts:

- a collar that reads and pushes sensors data
- a web app that allows reading and analysing pets data
- a database migration tool

Quick Start
---

Language specific implementation resides in the corresponding folder:

- [go](go)
- rust
- java
- cpp
- python
- node.js

The project uses `docker-compose` to provide simplest local
deployment of the ScyllaDB database cluster.

Architecture
---

```
Pet -> Sensors -> Collar -> ScyllaDB <---- WebApp <- User
                               ^             |
                               \-aggregation-/
```

In this very simple example a Collar generates sensors data
once a second and send it directly to the database.

Pet owners, sensors and measurements data can be accessed via
the REST API web server. It also implements lazy-evaluation
of the data aggregates.

Links
---

- [ScyllaDB Docker image](https://hub.docker.com/r/scylladb/scylla/)
- [ScyllaDB drivers page](https://docs.scylladb.com/using-scylla/scylla_drivers/)
