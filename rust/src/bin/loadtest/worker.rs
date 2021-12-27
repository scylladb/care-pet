use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use log::*;
use scylla::Session;
use uuid::Uuid;

use crate::stats::Stats;
use care_pet::db::TABLE_MEASUREMENT;
use care_pet::duration::Duration;
use care_pet::{insert_query, random_sensor_data, Measure, Sensor};

pub async fn work(sess: &'static Session, id: usize, sensors: Arc<HashMap<Uuid, Vec<Sensor>>>) {
    debug!("worker # {} ready", id);

    let prefix = format!("#{}", id);
    let mut s = Stats::new();

    loop {
        for (_, sensors) in sensors.iter() {
            for sensor in sensors {
                let measure = Measure {
                    sensor_id: sensor.sensor_id,
                    ts: Duration::now(),
                    value: random_sensor_data(sensor),
                };

                let query = insert_query!(TABLE_MEASUREMENT, Measure);

                let ts = Instant::now();
                sess.query(query, &measure)
                    .await
                    .map(|_| trace!("worker # {} insert {:?}", id, &measure))
                    .map_err(|err| error!("worker # {} error {:?}", id, err))
                    .ok();

                s.record(ts);
            }

            s.print(&prefix);
        }
    }
}
