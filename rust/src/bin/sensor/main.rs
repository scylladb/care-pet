use std::time::{self, Instant};

use log::*;
use scylla::batch::Batch;
use scylla::Session;
use structopt::StructOpt;
use tokio::time::sleep;

use care_pet::insert_query;
use care_pet::model::duration::Duration;
use care_pet::model::*;
use care_pet::{database, Result};

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
    db_config: database::Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    care_pet::log::init();

    let app = App::from_args();
    debug!("Configuration = {:?}", &app);

    info!("Welcome to the Pet collar simulator");

    let sess = database::new_session_with_keyspace(&app.db_config).await?;

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
    sess.query(insert_query!(Owner), owner).await?;
    info!("New owner # {}", owner.owner_id);

    sess.query(insert_query!(Pet), pet).await?;
    info!("New pet # {}", pet.pet_id);

    for sensor in sensors {
        sess.query(insert_query!(Sensor), sensor).await?;
    }

    Ok(())
}

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

fn read_sensor_data(sensor: &Sensor) -> Measure {
    Measure {
        sensor_id: sensor.sensor_id,
        ts: Duration::now(),
        value: random_sensor_data(sensor),
    }
}
