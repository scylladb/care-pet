use log::*;
use structopt::StructOpt;

use care_pet::{db, Result};

#[derive(Debug, StructOpt)]
#[structopt(name = "migrate")]
struct App {
    #[structopt(flatten)]
    db_config: db::Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    care_pet::log::init();

    let app = App::from_args();
    debug!("Configuration = {:?}", app);

    info!("Bootstrapping database...");

    let sess = db::new_session(&app.db_config).await?;

    db::create_keyspace(&sess).await?;
    db::migrate(&sess).await?;

    Ok(())
}
