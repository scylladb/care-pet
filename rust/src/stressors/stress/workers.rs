use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use log::*;
use uuid::Uuid;

use crate::model::sensors::sensor::Sensor;
use crate::model::sensors::sensor_measure::Measure;
use crate::repositories::Repositories;
use crate::stressors::stress::flock::Flock;
use crate::stressors::stress::stats::Stats;

pub async fn start_pets_worker(
    repositories: Arc<Repositories>,
    interval: Duration,
    mut flock: Flock
) -> Result<()>
{
    for pet in flock.pets {
        let sensors = flock.sensors.remove(&pet.pet_id).unwrap();

        info!("pet # {} ready", pet.pet_id);

        let repo = Arc::clone(&repositories);
        tokio::spawn(async move {
            loop {
                for sensor in &sensors {
                    let measure = Measure::new_from_sensor(sensor);
                    repo.sensor.create_measure(measure).await.unwrap();
                    info!("pet # {} insert", pet.pet_id)
                }
                tokio::time::sleep(interval.clone()).await;
            }
        });

    }
    Ok(())
}


pub async fn start_random_worker (repositories: Arc<Repositories>, id: usize, sensors: Arc<HashMap<Uuid, Vec<Sensor>>>) {
    debug!("worker # {} ready", id);

    let prefix = format!("#{}", id);
    let mut s = Stats::new();

    loop {
        for sensors in sensors.values() {
            for sensor in sensors {
                let measure = Measure::new_from_sensor(sensor);
                repositories.sensor.create_measure(measure.clone()).await.unwrap();

                let ts = Instant::now();
                trace!("worker # {} insert {:?}", id, measure.sensor_id);

                s.record(ts);
            }

            s.print(&prefix);
        }
    }
}
