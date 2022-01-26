mod cql;

use itertools::Itertools;
use log::*;
use scylla::{Session, SessionBuilder};
use structopt::StructOpt;

use crate::Result;

pub static KEYSPACE: &str = "carepet";
pub static TABLE_OWNER: &str = "owner";
pub static TABLE_PET: &str = "pet";
pub static TABLE_SENSOR: &str = "sensor";
pub static TABLE_MEASUREMENT: &str = "measurement";
pub static TABLE_SENSOR_AVG: &str = "sensor_avg";

#[derive(Debug, StructOpt)]
pub struct Config {
    // Cluster nodes address list
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub hosts: Vec<String>,

    // Connection timeout
    #[structopt(short, long, default_value = "60s")]
    pub timeout: humantime::Duration,

    // Password based authentication username
    #[structopt(short, long)]
    pub username: Option<String>,

    // Password based authentication password
    #[structopt(short, long)]
    pub password: Option<String>,
}

pub async fn new_session(config: &Config) -> Result<Session> {
    info!("Connecting to {}", config.hosts.join(", "));

    SessionBuilder::new()
        .known_nodes(&config.hosts)
        .connection_timeout(config.timeout.into())
        .user(
            config.username.clone().unwrap_or_default(),
            config.password.clone().unwrap_or_default(),
        )
        .build()
        .await
        .map_err(From::from)
}

pub async fn new_session_with_keyspace(config: &Config) -> Result<Session> {
    let session = new_session(config).await?;
    session.use_keyspace(KEYSPACE, true).await?;
    Ok(session)
}

pub async fn create_keyspace(sess: &Session) -> Result<()> {
    info!("Creating keyspace {}", KEYSPACE);

    sess.query(cql::KEYSPACE, ())
        .await
        .map(|_| info!("Keyspace {} created", KEYSPACE))
        .map_err(From::from)
}

pub async fn migrate(sess: &Session) -> Result<()> {
    info!("Migrating database");

    let queries_count = cql::MIGRATE.len();
    for (i, query) in cql::MIGRATE.iter().map(AsRef::as_ref).enumerate() {
        debug!("migrate = {}", query);
        sess.query(query, ())
            .await
            .map(|_| info!("Executed migration script {}/{}", i + 1, queries_count))?;
    }

    info!("Database migrated");

    Ok(())
}

pub fn fields(f: &[&'static str]) -> String {
    f.iter().map(|f| f.replace("r#", "")).join(", ")
}

pub fn values<const N: usize>() -> String {
    [0; N].map(|_| "?").join(", ")
}

#[macro_export]
macro_rules! insert_query {
    ($T:ty) => {{
        use $crate::model::ModelTable;

        scylla::query::Query::new(format!(
            "INSERT INTO {} ({}) VALUES ({})",
            <$T>::table(),
            $crate::db::fields(<$T>::FIELD_NAMES_AS_ARRAY),
            $crate::db::values::<{ <$T>::FIELD_NAMES_AS_ARRAY.len() }>()
        ))
    }};
}
