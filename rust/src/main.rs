use std::sync::Arc;
use actix_web::{web, HttpResponse, HttpServer};
use actix_web::web::Data;
use log::*;
use scylla::Session;

use structopt::StructOpt;
use care_pet::{AppState, db};

mod model;

mod controllers;

mod repositories;

#[derive(Debug, StructOpt)]
#[structopt(name = "care-pet")]
struct App {
    // Output more info
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(flatten)]
    db_config: db::Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    care_pet::log::init();

    let app = App::from_args();
    if app.verbose {
        info!("Configuration = {:?}", app);
    }

    let sess = Arc::new(db::new_session_with_keyspace(&app.db_config).await.unwrap());

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::new(AppState {
                session: Arc::clone(&sess),
            }))
            .service(controllers::owner_controller::index)
            .service(controllers::pets_controller::find_pets_by_owner_id)
            .service(controllers::sensors_controller::find_sensors_by_pet_id)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}