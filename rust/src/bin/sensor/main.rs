use std::time::{self, Instant};

use log::*;
use scylla::batch::Batch;
use scylla::query::Query;
use scylla::Session;
use structopt::StructOpt;
use tokio::time::sleep;

use care_pet::db::{TABLE_MEASUREMENT, TABLE_OWNER, TABLE_PET, TABLE_SENSOR};
use care_pet::model::duration::Duration;
use care_pet::model::*;
use care_pet::{db, Result};

#[derive(Debug, StructOpt)]
#[structopt(name = "migrate")]
struct App {
    // Sensors measurement interval
    #[structopt(short, long, default_value = "1h")]
    buffer_interval: humantime::Duration,

    // Buffer to accumulate measures
    #[structopt(short, long, default_value = "60s")]
    measure: humantime::Duration,

    #[structopt(flatten)]
    db_config: db::Config,
}

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

fn random_data() -> (Owner, Pet, Vec<Sensor>) {
    let owner = Owner::random();
    let pet = Pet::random(&owner);
    let sensors = (0..=rand::random::<usize>() & SensorType::len())
        .map(|_| Sensor::random(&pet))
        .collect();

    (owner, pet, sensors)
}

async fn save_data(sess: &Session, owner: &Owner, pet: &Pet, sensors: &[Sensor]) -> Result<()> {
    sess.query(
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            TABLE_OWNER,
            db::fields(Owner::FIELD_NAMES_AS_ARRAY),
            db::values::<{ Owner::FIELD_NAMES_AS_ARRAY.len() }>()
        ),
        owner.clone(),
    )
    .await?;
    info!("New owner # {}", owner.owner_id);

    sess.query(
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            TABLE_PET,
            db::fields(Pet::FIELD_NAMES_AS_ARRAY),
            db::values::<{ Pet::FIELD_NAMES_AS_ARRAY.len() }>()
        ),
        pet.clone(),
    )
    .await?;
    info!("New pet # {}", pet.pet_id);

    for sensor in sensors.iter().cloned() {
        sess.query(
            format!(
                "INSERT INTO {} ({}) VALUES ({})",
                TABLE_SENSOR,
                db::fields(Sensor::FIELD_NAMES_AS_ARRAY),
                db::values::<{ Sensor::FIELD_NAMES_AS_ARRAY.len() }>()
            ),
            sensor.clone(),
        )
        .await?;
    }

    Ok(())
}

async fn run_sensor_data(cfg: &App, sess: &Session, sensors: Vec<Sensor>) -> Result<()> {
    let measure: time::Duration = cfg.measure.into();
    let buffer_interval: time::Duration = cfg.buffer_interval.into();

    let mut last = Instant::now();
    let mut measures = vec![];
    loop {
        while last.elapsed() < buffer_interval {
            sleep(measure).await;

            for sensor in sensors.iter() {
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

        let (batch, values) = measures.drain(..).fold(
            (Batch::default(), vec![]),
            |(mut batch, mut values), measure| {
                let query = Query::new(format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    TABLE_MEASUREMENT,
                    db::fields(Measure::FIELD_NAMES_AS_ARRAY),
                    db::values::<{ Measure::FIELD_NAMES_AS_ARRAY.len() }>()
                ));

                batch.append_statement(query);
                values.push(measure);
                (batch, values)
            },
        );

        sess.batch(&batch, values)
            .await
            .map_err(|err| error!("execute batch query {:?}", err))
            .ok();
    }
}

fn read_sensor_data(sensor: &Sensor) -> Measure {
    Measure {
        sensor_id: sensor.sensor_id,
        ts: Duration::now(),
        value: random_sensor_data(sensor),
    }
}
