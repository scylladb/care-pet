use anyhow::Result;
use log::info;
use scylla::{Session, SessionBuilder};

use crate::cli::ServerConfig;

pub mod migrate;

pub async fn new_session(config: &ServerConfig) -> Result<Session> {
    info!("Connecting to {}", config.hostnames.join(", "));

    let session = SessionBuilder::new()
        .known_nodes(&config.hostnames)
        .connection_timeout(config.timeout)
        .user(
            &config.username,
            &config.password,
        )
        .build()
        .await?;

    Ok(session)
}

pub async fn new_session_with_keyspace(config: &ServerConfig) -> Result<Session> {
    let session = new_session(config).await?;
    session.use_keyspace(&config.keyspace, true).await?;
    Ok(session)
}
