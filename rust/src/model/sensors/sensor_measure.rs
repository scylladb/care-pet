use charybdis::macros::scylla::FromRow;
use chrono::{DateTime, Utc};
use scylla::ValueList;
use serde::Serialize;
use uuid::Uuid;
use crate::model::sensors::sensor::Sensor;

#[derive(Clone, ValueList, FromRow, Serialize)]
pub struct Measure {
    pub sensor_id: Uuid,
    pub ts: DateTime<Utc>,
    pub value: f32,
}

impl Measure {
    pub fn new_from_sensor(sensor: &Sensor) -> Self {
        Self {
            sensor_id: sensor.sensor_id,
            ts: Utc::now(),
            value: sensor.random_sensor_data(),
        }
    }
}
