use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use log::*;
use tokio::time::sleep;

use crate::cli::ServerConfig;
use crate::database;
use crate::model::owner::Owner;
use crate::model::pet::Pet;
use crate::model::sensors::sensor::Sensor;
use crate::model::sensors::sensor_measure::Measure;
use crate::repositories::Repositories;

pub async fn sensor_stress(config: &ServerConfig, measure: &Duration, buffer_interval: &Duration) -> Result<()> {
    info!("Welcome to the Pet collar simulator");

    let sess = Arc::new(database::new_session_with_keyspace(&config).await?);
    let repositories = Repositories::new(Arc::clone(&sess)).await;

    let (owner, pet, sensors) = random_data();
    save_data(&repositories, &owner, &pet, &sensors).await?;
    run_sensor_data(&repositories, sensors, measure, buffer_interval).await?;

    Ok(())
}

async fn run_sensor_data(
    repositories: &Repositories,
    sensors: Vec<Sensor>,
    measure: &Duration,
    buffer_interval: &Duration
) -> Result<()> {

    let mut last = Instant::now();
    loop {
        let mut measures = vec![];
        while last.elapsed() < *buffer_interval {
            sleep(*measure).await;

            for sensor in &sensors {
                let measure = Measure::new_from_sensor(sensor);
                info!(
                    "sensor # {} type {} new measure {} ts {}",
                    sensor.sensor_id,
                    sensor.sensor_type.as_str(),
                    &measure.value,
                    measure.ts.format("%Y-%m-%d %H:%M:%S").to_string(),
                );

                measures.push(measure);
            }
        }

        last = last + Duration::from_nanos(
            (measure.as_nanos() * (last.elapsed().as_nanos() / measure.as_nanos())) as u64,
        );

        info!("Pushing data");

        for measure in measures {
            repositories.sensor.create_measure(measure).await?;
        }
    }
}

async fn save_data(
    repositories: &Repositories,
    owner: &Owner,
    pet: &Pet,
    sensors: &[Sensor]
) -> Result<()> {
    repositories.owner.create(owner.clone()).await?;
    info!("New owner # {}", owner.owner_id);

    repositories.pet.create(pet.clone()).await?;
    info!("New pet # {}", pet.pet_id);

    for sensor in sensors {
        repositories.sensor.create(sensor.clone()).await?;
        info!("New sensor # {}", sensor.sensor_id);
    }

    Ok(())
}

fn random_data() -> (Owner, Pet, Vec<Sensor>) {
    let owner = Owner::random();
    let pet = Pet::random(&owner);
    let sensors = [1,2,3,4]
        .iter()
        .map(|_| Sensor::random(&pet))
        .collect();

    (owner, pet, sensors)
}
