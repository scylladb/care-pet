mod flock;
mod pets;
mod stats;
mod worker;

use std::sync::Arc;

use log::*;
use scylla::Session;
use structopt::StructOpt;

use care_pet::{database, Result};

#[derive(Debug, StructOpt)]
#[structopt(name = "loadtest")]
struct App {
    // number of the startPets owners
    #[structopt(long, default_value = "100")]
    owners: usize,

    // number of startPets to simulate
    #[structopt(long, default_value = "100")]
    pets: usize,

    // number of sensors per pet
    #[structopt(long, default_value = "4")]
    sensors: usize,

    // an interval between sensors measurements
    #[structopt(long, default_value = "1s")]
    interval: humantime::Duration,

    // just write random data
    #[structopt(long)]
    writer: bool,

    // number of parallel writers: Default: num of CPUs
    #[structopt(long)]
    workers: Option<usize>,

    #[structopt(flatten)]
    db_config: database::Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    care_pet::log::init();

    let app: App = App::from_args();
    debug!("Configuration = {:?}", app);

    info!("Welcome to the Pets simulator");
    rlimit::utils::increase_nofile_limit(102400)
        .map_err(|err| error!("unable to increase NOFILE limit: {:?}", err))
        .ok();

    // an easy way to get a static lifetime Session without wrapping it in an Arc
    // this is fine since we know that the session will last for the lifetime
    // of the program and that the Session is thread safe
    let sess: &'static Session = Box::leak(Box::new(
        database::new_session_with_keyspace(&app.db_config).await?,
    ));

    info!("Creating flock");
    let f = flock::Flock::new(app.owners, app.pets, app.sensors);

    f.save(sess).await?;

    let workers = app.workers.unwrap_or_else(num_cpus::get);
    if app.writer {
        let sensors = Arc::new(f.sensors);
        for i in 1..=workers {
            let sensors = sensors.clone();
            tokio::spawn(async move {
                worker::work(sess, i, sensors).await;
            });
        }

        info!("Writers started");
    } else {
        pets::start(sess, app.interval, f);
        info!("Flock started");
    }

    let () = std::future::pending().await;
    Ok(())
}
