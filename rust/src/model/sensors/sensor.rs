use rand::Rng;
use scylla::{FromRow, SerializeRow};
use serde::Serialize;
use uuid::Uuid;

use crate::model::pet::Pet;
use crate::model::sensors::sensor_type::SensorType;

#[derive(Clone, SerializeRow, FromRow, Serialize)]
pub struct Sensor {
    pub pet_id: Uuid,
    pub sensor_id: Uuid,
    pub sensor_type: SensorType,
}

impl Sensor {
    pub fn random(p: &Pet) -> Self {
        Self {
            pet_id: p.pet_id,
            sensor_id: Uuid::new_v4(),
            sensor_type: SensorType::random(),
        }
    }

    pub fn random_sensor_data(&self) -> f32 {
        let mut rng = rand::thread_rng();

        match self.sensor_type {
            SensorType::Temperature => 101.0 + rng.gen_range(0.0..10.0) - 4.0,
            SensorType::Pulse => 101.0 + rng.gen_range(0.0..40.0) - 20.0,
            SensorType::Respiration => 35.0 + rng.gen_range(0.0..5.0) - 2.0,
            SensorType::Location => 10.0 * rand::random::<f32>(),
        }
    }
}
