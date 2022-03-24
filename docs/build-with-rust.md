## Build an IoT App with Rust

### Architecture

This section will walk through and explain the code for the different commands.
As explained in the Getting Started page, the project is structured as follow:

-   migrate (`/bin/migrate/main.rs`) - creates the `carepet` keyspace and tables
-   collar (`/bin/sensor/main.rs`) - generates a pet health data and pushes it into the storage
-   web app (`/main.rs`) - REST API service for tracking pets health state

### Migrate

Start by creating a local ScyllaDB cluster consisting of 3 nodes:

```
$ docker-compose up -d
```
Docker-compose will spin up a ScyllaDB cluster consisting of 3 nodes (carepet-scylla1, carepet-scylla2 and carepet-scylla3) along with the app (for example go-app) container.  Wait for about two minutes and check the status of the cluster: To check the status of the cluster:
```
$ docker exec -it carepet-scylla1 nodetool status
```
Once all the nodes are in UN - Up Normal status, run the below commands:

The below command allows you to get node IP address:

```
docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1
```

The run the following commands to execute the migrate main function.

```
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
cargo run --bin migrate -- --hosts $NODE1
```

The command executes the main function in the `bin/migrate/main.rs`. The function creates the keyspace and tables that you need to run the collar and server services.

The below code in the `bin/migrate/main.rs` creates a new session then calls the `create_keyspace` , `migrate` functions.

```
// migrate/main.rs

async fn main() -> Result<()> {
    care_pet::log::init();

    let app = App::from_args();
    debug!("Configuration = {:?}", app);

    info!("Bootstrapping database...");

    let sess = db::new_session(&app.db_config).await?;

    db::create_keyspace(&sess).await?;
    db::migrate(&sess).await?;

    Ok(())
}
```

The `new_session` function takes the config as a parameter and uses `SessionBuilder` class to crete a new session.

```
// db/mod.rs

pub async fn new_session(config: &Config) -> Result<Session> {
    info!("Connecting to {}", config.hosts.join(", "));

    SessionBuilder::new()
        .known_nodes(&config.hosts)
        .connection_timeout(config.timeout.into())
        .user(
            config.username.clone().unwrap_or_default(),
            config.password.clone().unwrap_or_default(),
        )
        .build()
        .await
        .map_err(From::from)
}
```

For more information about creating a new session with the Rust Driver, please have a look at the [docs](https://rust-driver.docs.scylladb.com/stable/quickstart/example.html).

`create_keyspace` function takes a session as an argument and creates a keyspace as defined in `db/keyspace.cql`:

```
CREATE KEYSPACE IF NOT EXISTS carepet WITH replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': '3' };
```

The CQL query above creates a new keyspace named carepet, with `NetworkTopologyStrategy` as replication strategy and a replication factor of 3.
More information about keyspace and replication on [Scylla University](https://university.scylladb.com/courses/data-modeling/lessons/basic-data-modeling-2/topic/keyspace/).

Finally, `migrate` will execute the queries listed in `db/migrate.cql` to create the tables you need for the project.

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

You can check the database structure with:

```
docker exec -it carepet-scylla1 cqlsh
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

```
$ NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
$ cargo run --bin sensor -- --hosts $NODE1 --measure 5s --buffer-interval 1m
```

The above command executes `bin/sensor/main.rs` and and takes the following as arguments.

-   `hosts` : the IP address of the ScyllaDB node.
-   `measure`: the interval between to sensor measures.
-   `buffer-interval`: the interval between two database queries.

```
// sensor/main.rs

#[tokio::main]
async fn main() -> Result<()> {
    care_pet::log::init();

    let app = App::from_args();
    debug!("Configuration = {:?}", &app);

    info!("Welcome to the Pet collar simulator");

    let sess = db::new_session_with_keyspace(&app.db_config).await?;

    let (owner, pet, sensors) = random_data();
    save_data(&sess, &owner, &pet, &sensors).await?;
    run_sensor_data(&app, &sess, sensors).await?;

    Ok(())
}
```

The `app` object contains the command's arguments listed above.
We then create a new session `sess` using `new_session_with_keyspace` function defined in `db/mod.rs`:

```
// db/mod.rs

pub async fn new_session_with_keyspace(config: &Config) -> Result<Session> {
    let session = new_session(config).await?;
    session.use_keyspace(KEYSPACE, true).await?;
    Ok(session)
}
```

The `save_data` method connects to the datbase and saves random `owner`, `pet` and the `sensors` to the database using `insert_query` macro defined in `src/mod.rs`.

```
// sensor/main.rs

async fn save_data(sess: &Session, owner: &Owner, pet: &Pet, sensors: &[Sensor]) -> Result<()> {
    sess.query(insert_query!(Owner), owner).await?;
    info!("New owner # {}", owner.owner_id);

    sess.query(insert_query!(Pet), pet).await?;
    info!("New pet # {}", pet.pet_id);

    for sensor in sensors {
        sess.query(insert_query!(Sensor), sensor).await?;
    }

    Ok(())
}
```

The `run_sensor_data` generates random data and inserts it to the database every `buffer_interval`.

```
async fn run_sensor_data(cfg: &App, sess: &Session, sensors: Vec<Sensor>) -> Result<()> {
    let measure: time::Duration = cfg.measure.into();
    let buffer_interval: time::Duration = cfg.buffer_interval.into();

    let mut last = Instant::now();
    loop {
        let mut measures = vec![];
        while last.elapsed() < buffer_interval {
            sleep(measure).await;

            for sensor in &sensors {
                let measure = read_sensor_data(sensor);
                info!(
                    "sensor # {} type {} new measure {} ts {}",
                    sensor.sensor_id,
                    sensor.r#type.as_str(),
                    &measure.value,
                    measure.ts.format_rfc3339(),
                );

                measures.push(measure);
            }
        }

        last = last
            + time::Duration::from_nanos(
                (measure.as_nanos() * (last.elapsed().as_nanos() / measure.as_nanos())) as u64,
            );

        info!("Pushing data");

        let batch = measures.iter().fold(Batch::default(), |mut batch, _| {
            batch.append_statement(insert_query!(Measure));
            batch
        });

        sess.batch(&batch, measures)
            .await
            .map_err(|err| error!("execute batch query {:?}", err))
            .ok();
    }
}
```

### Server

The server service is a REST API for tracking the pets’ health state. The service was built using [Rocket](https://rocket.rs) and allows users to query the database via HTTP.

Run the following commands to start the server:
```
NODE1=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' carepet-scylla1)
cargo run -- --hosts $NODE1
```

The `src/main.rs` main function mounts the api on `/api` and defines the routes.

```
// src/main.rs

#[rocket::main]
async fn main() -> Result<()> {
    care_pet::log::init();

    let app = App::from_args();
    if app.verbose {
        info!("Configuration = {:?}", app);
    }

    let sess = db::new_session_with_keyspace(&app.db_config).await?;

    rocket::build()
        .mount(
            "/api",
            routes![
                handler::measures::find_sensor_data_by_sensor_id_and_time_range,
                handler::owner::find_owner_by_id,
                handler::pets::find_pets_by_owner_id,
                handler::sensors::find_sensors_by_pet_id,
                handler::avg::find_sensor_avg_by_sensor_id_and_day
            ],
        )
        .manage(sess)
        .launch()
        .await
        .map_err(From::from)
}
```

The handlers can be found in the `src/handler` folder for each route.

Let's have a look at `handler/mesure.rs` file:

```
#[get("/sensor/<id>/values?<from>&<to>")]
pub async fn find_sensor_data_by_sensor_id_and_time_range(
    session: &State<Session>,
    id: UuidParam,
    from: DateTimeParam,
    to: DateTimeParam,
) -> Result<Json<Vec<f32>>, JsonError> {
    let rows = session
        .query(
            format!(
                "SELECT {} FROM {} WHERE {} = ? and {} >= ? and {} <= ?",
                Measure::FIELD_NAMES.value,
                Measure::table(),
                Measure::FIELD_NAMES.sensor_id,
                Measure::FIELD_NAMES.ts,
                Measure::FIELD_NAMES.ts,
            ),
            (id.0, from.0, to.0),
        )
        .await
        .map_err(|err| json_err(Status::InternalServerError, err))?
        .rows
        .unwrap_or_default()
        .into_typed::<(f32,)>();

    let values = rows
        .map(|v| v.map(|v| v.0))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| json_err(Status::InternalServerError, err))?;

    Ok(Json(values))
}
```

The GET request on URL `/sensor/<id>/values?<from>&<to>` triggers `find_sensor_data_by_sensor_id_and_time_range` function.

`find_sensor_data_by_sensor_id_and_time_range` takes `session`, `id`, `from` and `to` as params. The function runs a `SELECT` query then returns `rows`.



To test out the API in your terminal, use the following command: `$ curl http://127.0.0.1:8000/api/owner/{id}` and expect the following response:

````

[{"address":"home","age":57,"name":"tlmodylu","owner_id":"a05fd0df-0f97-4eec-a211-cad28a6e5360","pet_id":"a52adc4e-7cf4-47ca-b561-3ceec9382917","weight":5}]

```

```
