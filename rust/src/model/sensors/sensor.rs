use rand::Rng;
use scylla::{FromRow, ValueList};
use serde::Serialize;
use struct_field_names::StructFieldNames;
use struct_field_names_as_array::FieldNamesAsArray;
use uuid::Uuid;

use crate::model::pet::Pet;
use crate::model::sensors::sensor_type::SensorType;

#[derive(Clone, ValueList, FromRow, Serialize, StructFieldNames, FieldNamesAsArray)]
pub struct Sensor {
    pub pet_id: Uuid,
    pub sensor_id: Uuid,
    pub r#type: SensorType,
}

impl Sensor {
    pub fn random(p: &Pet) -> Self {
        Self {
            pet_id: p.pet_id,
            sensor_id: Uuid::new_v4(),
            r#type: SensorType::random(),
        }
    }

    pub fn random_sensor_data(&self) -> f32 {
        let mut rng = rand::thread_rng();

        match self.r#type {
            SensorType::Temperature => 101.0 + rng.gen_range(0.0..10.0) - 4.0,
            SensorType::Pulse => 101.0 + rng.gen_range(0.0..40.0) - 20.0,
            SensorType::Respiration => 35.0 + rng.gen_range(0.0..5.0) - 2.0,
            SensorType::Location => 10.0 * rand::random::<f32>(),
        }
    }
}
