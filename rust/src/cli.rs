use std::time::Duration;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the Actix Web Server
    Server(ServerConfig),
    Migrate {
        #[command(flatten)]
        config: ServerConfig,

        #[arg(short, long, default_value = "false")]
        drop_keyspace: bool,
    },
    /// Run the sample data generator for sensors
    Sensor {
        #[command(flatten)]
        config: ServerConfig,

        #[arg(short, long, default_value = "60", value_parser = parse_duration)]
        measure: Duration,

        #[arg(short, long, default_value = "3600", value_parser = parse_duration)]
        buffer_interval: Duration,
    },
    /// Run the stressing test for the application
    Stress {
        #[command(flatten)]
        config: ServerConfig,

        #[command(flatten)]
        stress: StressConfig,
    }
}

#[derive(Args)]
pub struct StressConfig {
    #[arg(short, long, default_value = "100")]
    pub owners: usize,

    #[arg(short, long, default_value = "100", short = 'e')]
    pub pets: usize,

    #[arg(short, long, default_value = "1", value_parser = parse_duration)]
    pub interval: Duration,

    #[arg(short, long, default_value = "4")]
    pub sensors: usize,

    #[arg(short, long, default_value = "4")]
    pub workers: usize,

    #[arg(short, long, default_value = "false")]
    pub random_data: bool,
}

#[derive(Args)]
pub struct ServerConfig {
    #[arg(short, long, default_value = "carepet")]
    pub keyspace: String,

    #[arg(long, default_value = "localhost:9042")]
    pub hostnames: Vec<String>,

    #[arg(short, long, default_value = "")]
    pub username: String,

    #[arg(short, long, default_value = "")]
    pub password: String,

    #[arg(short, long, default_value = "2", value_parser = parse_duration)]
    pub timeout: Duration,
}

fn parse_duration(arg: &str) -> Result<Duration, std::num::ParseIntError> {
    let secs = arg.parse()?;
    Ok(Duration::from_secs(secs))
}
