use std::sync::Arc;

use actix_web::HttpServer;
use actix_web::web::Data;

use crate::{AppState, database};
use crate::cli::ServerConfig;
use anyhow::Result;
pub mod controllers;

pub async fn start_server(args: &ServerConfig) -> Result<()> {

    let session = Arc::new(database::new_session_with_keyspace(args).await.unwrap());

    let _ = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::new(AppState {
                session: Arc::clone(&session),
            }))
            .service(controllers::owner_controller::index)
            .service(controllers::pets_controller::find_pets_by_owner_id)
            .service(controllers::sensors_controller::find_sensors_by_pet_id)
            .service(controllers::sensors_controller::find_sensor_avg_by_sensor_id_and_day)
            .service(controllers::sensors_controller::find_sensor_data_by_sensor_id_and_time_range)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await;

    Ok(())
}
