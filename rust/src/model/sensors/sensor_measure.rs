use charybdis::macros::scylla::{FromRow, SerializeRow};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::model::sensors::sensor::Sensor;

#[derive(Clone, SerializeRow, FromRow, Serialize)]
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
