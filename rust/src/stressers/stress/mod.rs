use std::sync::Arc;
use log::{error, info};
use crate::cli::{ServerConfig, StressConfig};
use anyhow::Result;
use rlimit::increase_nofile_limit;
use crate::database::new_session_with_keyspace;
use crate::repositories::Repositories;

mod flock;
mod stats;
mod workers;

pub async fn application_stress(
    config: &ServerConfig,
    stress_config: &StressConfig
) -> Result<()> {

    info!("Welcome to the Pets simulator");

    increase_nofile_limit(102400)
        .map_err(|err| error!("unable to increase NOFILE limit: {:?}", err))
        .ok();

    let session = Arc::new(new_session_with_keyspace(config).await?);
    let repositories = Arc::new(Repositories::new(Arc::clone(&session)).await);

    info!("Creating flock");
    let flock = flock::Flock::new(stress_config);

    flock.save(&repositories).await?;

    let workers = stress_config.workers;

    if stress_config.random_data {
        let sensors = Arc::new(flock.sensors);
        for i in 1..=workers {
            let sensors = sensors.clone();
            let repositories = Arc::clone(&repositories);
            tokio::spawn(async move {
                workers::start_random_worker(repositories, i, sensors).await;
            });
        }

        info!("Writers started");
    } else {
        workers::start_pets_worker(Arc::clone(&repositories), stress_config.interval, flock).await?;
        info!("Flock started");
    }

    let () = std::future::pending().await;
    Ok(())
}
