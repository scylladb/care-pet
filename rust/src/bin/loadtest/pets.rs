use log::*;
use scylla::Session;

use crate::flock::Flock;

use care_pet::duration::Duration;
use care_pet::insert_query;
use care_pet::model::{random_sensor_data, Measure};

pub fn start(sess: &'static Session, interval: humantime::Duration, mut f: Flock) {
    for pet in f.1 {
        let sensors = f.2.remove(&pet.pet_id).unwrap();

        debug!("pet # {} ready", pet.pet_id);

        tokio::spawn(async move {
            loop {
                for sensor in &sensors {
                    let measure = Measure {
                        sensor_id: sensor.sensor_id,
                        ts: Duration::now(),
                        value: random_sensor_data(sensor),
                    };

                    let query = insert_query!(Measure);
                    sess.query(query, measure)
                        .await
                        .map(|_| trace!("pet # {} insert", pet.pet_id))
                        .map_err(|err| error!("pet # {} error {:?}", pet.pet_id, err))
                        .ok();
                }

                tokio::time::sleep(interval.into()).await;
            }
        });
    }
}
