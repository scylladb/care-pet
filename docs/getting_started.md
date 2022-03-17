Getting Started with CarePet: A sample IoT App
----------------------------------------------

### Introduction

This guide will show you how to create an IoT app from scratch and configure it
to use Scylla as the backend datastore. It'll walk you through all the stages
of the development process, from gathering requirements to building and running
the application.

As an example, you will use an application called CarePet. CarePet allows pet
owners to track their pets' health by monitoring their key health parameters,
such as temperature or pulse. The application consists of three parts:
-   A pet collar with sensors that collects pet health data and sends the data to the datastore.
-   A web app for reading the data and analyzing the pets' health.
-   A database migration module.

You can use this example with minimal changes for any IoT application. 

### Architecture

-  `migrate` - Creates the CarePet keyspace and tables.
-  `sensor` - Generates pet health data and pushes it into storage.
-  `server` - REST API service for tracking the pets' health state.

![Build your first ScyllaDB Powered App - Raouf](https://user-images.githubusercontent.com/13738772/158378310-11a39630-b390-4df0-8096-2c1751e56570.jpg)


### Requirements

#### Prerequisites for Deploying the Application

The example application uses Docker to run a three-node ScyllaDB cluster. You can also use Scylla Cloud as your database.
Claim your free Scylla Cloud account [here](https://scylladb.com/cloud).

#### Use Case Requirements

Each pet collar has sensors that report four different measurements:
temperature, pulse, location, and respiration.

The collar reads the measurements from the sensors once per second
and sends the data directly to the app.

#### Performance Requirements

The application has two performance-related parts: sensors that write to
the database (throughput sensitive) and a backend dashboard that reads from
the database (latency sensitive). 

* This example assumes 99% writes (sensors) and 1% reads (backend dashboard).  
* SLA:
  - Writes: throughput of 100K operations per second.
  - Reads: latency of up to 10 milliseconds for the
    [99th percentile](https://engineering.linkedin.com/performance/who-moved-my-99th-percentile-latency).
* The application requires high availability and fault tolerance. Even if a
ScyllaDB node goes down or becomes unavailable, the cluster is expected to
remain available and continue to provide service. You can learn more about
Scylla high availability in [this lesson](https://university.scylladb.com/courses/scylla-essentials-overview/lessons/high-availability/). 


### Deploying the Application

1. Check out the care-pet repository from GitHub and go to the directory of
   the language you want to use (Go, Java, etc.):
    
    ```
    git clone git@github.com:scylladb/care-pet.git
    cd go
    ```

1. Create a local ScyllaDB cluster consisting of three nodes:

    ```
    docker-compose up -d
    ```

   Docker-compose will spin up a ScyllaDB cluster consisting of three nodes:
   carepet-scylla1, carepet-scylla2 and carepet-scylla3. The process will
   take about two minutes.

1. Check the status of the cluster:

    ```
    docker exec -it carepet-scylla1 nodetool status
    ```

1. Continue by following the instructions for the programming language you're using. See [Build the Application with Your Programming Language](#build-the-application-with-your-programming-language).

### Build the Application with Your Programming Language

- [Build with Go](/build-with-go.md)
- [Build with Java](/build-with-java.md)
- [Build with JavaScript](/build-with-javascript.md)
- [Build with Rust](/build-with-rust.md)


### Additional Resources

-   [Scylla Essentials](https://university.scylladb.com/courses/scylla-essentials-overview/) course on Scylla University. It provides an introduction to Scylla and explains the basics.
-   [Data Modeling and Application Development](https://university.scylladb.com/courses/data-modeling/) course on Scylla University. It explains basic and advanced data modeling techniques, including information on workflow application, query analysis, denormalization, and other NoSQL data modeling topics.
-   [Scylla Documentation](https://docs.scylladb.com/)
-   Scylla users [slack channel](http://slack.scylladb.com/)

Future Work

-   Add Sizing
-   Add Benchmarking
-   Add Python implementation
-   In a real-world application, it would be better to aggregate data in an internal buffer and send it once a day to the application gateway in a batch, implying techniques such as delta encoding. It could also aggregate data at a lower resolution and take measurements less frequently. The collar could notify the pet's owner about suspicious health parameters directly or via the application. 
-   Add location tracking info to send alerts when the pet enters/leaves safe zones using known WiFi networks.
-   Use the measurements to present to the pet owner health alerts, vital signs, sleeping levels, activity levels, and calories burned.
