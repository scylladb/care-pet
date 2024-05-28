#![allow(warnings)]

use clap::Parser;
use log::*;
use anyhow::Result;

use care_pet::cli::{Cli, Commands};
use care_pet::database::migrate::migrate;
use care_pet::http::start_server;
use care_pet::stressors::sensor::sensor_stress;
use care_pet::stressors::stress::application_stress;


#[actix_web::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    care_pet::log::init();

    match &cli.command {
        Commands::Server(args)
            => start_server(args).await,
        Commands::Migrate { config, drop_keyspace }
            => migrate(config, drop_keyspace.clone()).await,
        Commands::Sensor { config, measure, buffer_interval }
            => sensor_stress(config, measure, buffer_interval).await,
        Commands::Stress { config, stress }
            => application_stress(config, stress).await,
    }
}
