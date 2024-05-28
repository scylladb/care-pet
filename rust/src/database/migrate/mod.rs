use std::path::{Path, PathBuf};
use crate::cli::ServerConfig;
use anyhow::Result;
use log::info;
use scylla::query::Query;
use scylla::Session;
use crate::database::new_session;

pub async fn migrate(server_config: &ServerConfig, should_drop_keyspace: bool) -> Result<()>
{
    let session = new_session(server_config).await?;

    if should_drop_keyspace {
        drop_keyspace(&session, server_config.keyspace.as_str()).await?;
    }

    create_keyspace(&session, &server_config.keyspace.as_str()).await?;
    create_tables(&session).await?;

    Ok(())
}

async fn create_tables(session: &Session) -> Result<()> {
    let current_path = std::env::current_dir().unwrap();
    let file_path = Path::new("src/database/migrate/migrate.cql");
    let full_path = current_path.join(file_path);

    let raw_queries = read_file(full_path)
        .await
        .unwrap_or_else(|_| panic!("Could not read file"));

    let queries = raw_queries.split(";")
        .map(|query| query.trim())
        .collect::<Vec<&str>>();


    info!("Creating tables...");
    for query in queries {
        if query.is_empty() {
            continue;
        }

        session.query(query, []).await?;
    }
    info!("Migration completed!");
    Ok(())
}

async fn drop_keyspace(session: &Session, keyspace: &str) -> Result<()> {
    let drop_query = format!("DROP KEYSPACE IF EXISTS {}", keyspace);
    let drop_query = Query::new(drop_query);
    session.query(drop_query, []).await?;

    Ok(())
}

pub async fn read_file(file: PathBuf) -> Result<String> {
    let content = tokio::fs::read_to_string(file).await?;
    Ok(content)
}

pub async fn create_keyspace(session: &Session, keyspace: &str) -> Result<()> {
    let keyspace_query = format!(
        "{} {} {}",
        "CREATE KEYSPACE IF NOT EXISTS",
        keyspace,
        "WITH replication = { 'class': 'NetworkTopologyStrategy', 'replication_factor': '3' }"
    );

    session.query(keyspace_query, []).await?;

    info!("Keyspace {} created", keyspace);
    Ok(())
}
