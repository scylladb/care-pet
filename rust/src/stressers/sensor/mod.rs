use std::sync::Arc;
use std::time::{self, Duration, Instant};

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::*;
use rand::Rng;
use scylla::batch::Batch;
use scylla::Session;
use tokio::time::sleep;
use crate::cli::ServerConfig;
use crate::database;
use crate::model::owner::Owner;
use crate::model::pet::Pet;
use crate::model::sensors::sensor::Sensor;
use crate::model::sensors::sensor_measure::Measure;
use crate::model::sensors::sensor_type::SensorType;
use crate::repositories::owner_repository::OwnerRepository;
use crate::repositories::pet_repository::PetRepository;
use crate::repositories::sensor_repository::SensorRepository;

pub struct Repositories {
    pub owner: OwnerRepository,
    pub pet: PetRepository,
    pub sensor: SensorRepository,
}

pub async fn sensor_stress(config: &ServerConfig, measure: &Duration, buffer_interval: &Duration) -> Result<()> {
    info!("Welcome to the Pet collar simulator");

    let sess = Arc::new(database::new_session_with_keyspace(&config).await?);
    let repositories = Repositories {
        owner: OwnerRepository::new(Arc::clone(&sess)).await,
        pet: PetRepository::new(Arc::clone(&sess)).await,
        sensor: SensorRepository::new(Arc::clone(&sess)).await,
    };

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
            sleep(measure.clone()).await;

            for sensor in &sensors {
                let measure = read_sensor_data(sensor);
                info!(
                    "sensor # {} type {} new measure {} ts {}",
                    sensor.sensor_id,
                    sensor.r#type.as_str(),
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

        repositories.sensor.batch_measures(measures).await?;
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
    let sensors = vec![1,2,3,4]
        .iter()
        .map(|_| Sensor::random(&pet))
        .collect();

    (owner, pet, sensors)
}

fn read_sensor_data(sensor: &Sensor) -> Measure {
    let mut rng = rand::thread_rng();

    let value = match sensor.r#type {
        SensorType::Temperature => 101.0 + rng.gen_range(0.0..10.0) - 4.0,
        SensorType::Pulse => 101.0 + rng.gen_range(0.0..40.0) - 20.0,
        SensorType::Respiration => 35.0 + rng.gen_range(0.0..5.0) - 2.0,
        SensorType::Location => 10.0 * rand::random::<f32>(),
    };

    Measure {
        sensor_id: sensor.sensor_id,
        ts: Utc::now(),
        value,
    }
}
