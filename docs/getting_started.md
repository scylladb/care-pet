Getting Started with CarePet: A sample IoT App
----------------------------------------------

### Introduction

In this guided exercise, you'll create an IoT app from scratch and configure it
to use ScyllaDB as the backend datastore.

We'll use as an example, an application called CarePet, which collects and
analyzes data from sensors attached to a pet's collar and monitors the pet's
health and activity.

The example can be used, with minimal changes, for any IoT like application.

We'll go over the different stages of the development, from gathering
requirements, creating the data model, cluster sizing and hardware needed to
match the requirements, and finally building and running the application. 

### Use Case Requirements

Each pet collar includes sensors that report four different measurements:
Temperature, Pulse, Location, and Respiration.

The collar reads the sensor's data once a second and sends measurements
directly to the app.

### Architecture

-   migrate - Creates the CarePet keyspace and tables.
-   sensor - generates a pet health data and pushes it into the
    storage.
-   server - REST API service for tracking the pets' health
    state.

![Build your first ScyllaDB Powered App - Raouf](https://user-images.githubusercontent.com/13738772/158378310-11a39630-b390-4df0-8096-2c1751e56570.jpg)


### Performance Requirements

The application has two parts:

-   Sensors: writes to the database, throughput sensitive
-   Backend dashboard: reads from the database, latency-sensitive

For this example, we assume 99% writes (sensors) and 1% reads (backend dashboard)

Required SLA:

-   Writes throughput of 100K Operations per second
-   Reads: latency of up to 10 milliseconds for the
    [99th percentile](https://engineering.linkedin.com/performance/who-moved-my-99th-percentile-latency).

The application requires high availability and fault tolerance. Even if a
ScyllaDB node goes down or becomes unavailable, the cluster is expected to
remain available and continue to provide service. You can learn more about
Scylla high availability in [this lesson](https://university.scylladb.com/courses/scylla-essentials-overview/lessons/high-availability/). 

### Design and Data Model

In this part  we’ll think about our queries, make the primary key and
clustering key selection, and create the database schema. See more in the data
model design [document](./design_and_data_model.md).

### Deploying the App 

The example application uses Docker to run a three-node ScyllaDB cluster. You can also use Scylla Cloud as your database.
Claim your free Scylla Cloud account [here](https://scylladb.com/cloud).

Clone the repository and change to the directory of a language (Go, Java, etc.):
```
git clone git@github.com:scylladb/care-pet.git
cd go
```

Start by creating a local ScyllaDB cluster consisting of 3 nodes:

`docker-compose up -d`

Docker-compose will spin up a ScyllaDB cluster consisting of 3 nodes (carepet-scylla1, carepet-scylla2 and carepet-scylla3) along with the app (for example go-app) container.  Wait for about two minutes and check the status of the cluster:
To check the status of the cluster:

`docker exec -it carepet-scylla1 nodetool status`

### Use your preffered programming language

- [Build with Go](/build-with-go.md)
- [Build with Java](/build-with-java.md)
- [Build with JavaScript](/build-with-javascript.md)
- [Build with Rust](/build-with-rust.md)


### Additional Resources

-   [Scylla Essentials](https://university.scylladb.com/courses/scylla-essentials-overview/) course on Scylla University. It provides an introduction to Scylla and explains the basics
-   [Data Modeling and Application Development](https://university.scylladb.com/courses/data-modeling/) course on Scylla University. It explains basic and advanced data modeling techniques, including information on workflow application, query analysis, denormalization, and other NoSQL data modeling topics.
-   [Scylla Documentation](https://docs.scylladb.com/)
-   Scylla users [slack channel](http://slack.scylladb.com/)

Future Work

-   Add Sizing
-   Add Benchmarking
-   Add Java implementation
-   Add Python implementation
-   In a real-world application, it would be better to aggregate data in an internal buffer and send it once a day to the application gateway in a batch, implying techniques such as delta encoding. It could also aggregate data at a lower resolution and take measurements less frequently. The collar could notify the pet's owner about suspicious health parameters directly or via the application. 
-   Add location tracking info to send alerts when the pet enters/leaves safe zones using known WiFi networks.
-   Use the measurements to present to the pet owner health alerts, vital signs, sleeping levels, activity levels, and calories burned.
