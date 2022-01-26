use log::*;
use rocket::routes;
use structopt::StructOpt;

use care_pet::{db, handler, result::Result};

#[derive(Debug, StructOpt)]
#[structopt(name = "care-pet")]
struct App {
    // Output more info
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(flatten)]
    db_config: db::Config,
}

#[rocket::main]
async fn main() -> Result<()> {
    care_pet::log::init();

    let app = App::from_args();
    if app.verbose {
        info!("Configuration = {:?}", app);
    }

    let sess = db::new_session_with_keyspace(&app.db_config).await?;

    rocket::build()
        .mount(
            "/api",
            routes![
                handler::measures::find_sensor_data_by_sensor_id_and_time_range,
                handler::owner::find_owner_by_id,
                handler::pets::find_pets_by_owner_id,
                handler::sensors::find_sensors_by_pet_id,
                handler::avg::find_sensor_avg_by_sensor_id_and_day
            ],
        )
        .manage(sess)
        .launch()
        .await
        .map_err(From::from)
}
