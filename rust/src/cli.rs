use std::time::Duration;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Server(ServerConfig),
    Migrate {
        #[command(flatten)]
        config: ServerConfig,

        #[arg(short, long, default_value = "false")]
        drop_keyspace: bool,
    },
    Sensor {
        #[command(flatten)]
        config: ServerConfig,

        #[arg(short, long, default_value = "60", value_parser = parse_duration)]
        measure: Duration,

        #[arg(short, long, default_value = "3600", value_parser = parse_duration)]
        buffer_interval: Duration,
    },
    Stress {
        #[command(flatten)]
        config: ServerConfig,
    }
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