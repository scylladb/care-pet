pub mod date;
pub mod duration;

use rand::prelude::*;
use rocket::serde::Serialize;
use scylla::cql_to_rust::{FromCqlVal, FromCqlValError};
use scylla::frame::response::result::CqlValue;
use scylla::frame::value::{Value, ValueTooBig};
use scylla::FromRow;
use scylla::{BufMut, ValueList};
use struct_field_names::StructFieldNames;
use struct_field_names_as_array::FieldNamesAsArray;
use uuid::Uuid;

use crate::date::Date;
use crate::db;
use crate::duration::Duration;

pub trait ModelTable {
    fn table() -> &'static str;
}

macro_rules! impl_model_table {
    ($table:expr, $T:ty) => {
        impl ModelTable for $T {
            fn table() -> &'static str {
                $table
            }
        }
    };
}

#[derive(Clone, ValueList, FromRow, Serialize, StructFieldNames, FieldNamesAsArray)]
pub struct Owner {
    pub owner_id: Uuid,
    pub address: String,
    pub name: String,
}

impl_model_table!(db::TABLE_OWNER, Owner);

impl Owner {
    pub fn random() -> Self {
        Self {
            owner_id: Uuid::new_v4(),
            address: "home".into(),
            name: random_string(8),
        }
    }
}

#[derive(
    Debug, Default, Clone, ValueList, FromRow, Serialize, StructFieldNames, FieldNamesAsArray,
)]
pub struct Pet {
    pub owner_id: Uuid,
    pub pet_id: Uuid,
    pub chip_id: Option<String>,
    pub species: Option<String>,
    pub breed: Option<String>,
    pub color: Option<String>,
    pub gender: Option<String>,
    pub age: i32,
    pub weight: f32,
    pub address: String,
    pub name: String,
}

impl_model_table!(db::TABLE_PET, Pet);

impl Pet {
    pub fn random(o: &Owner) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            owner_id: o.owner_id,
            pet_id: Uuid::new_v4(),
            age: rng.gen_range(1..100),
            weight: rng.gen_range(5.0..10.0),
            address: o.address.clone(),
            name: random_string(8),

            ..Default::default()
        }
    }
}

#[derive(Clone, ValueList, FromRow, Serialize, StructFieldNames, FieldNamesAsArray)]
pub struct Sensor {
    pub pet_id: Uuid,
    pub sensor_id: Uuid,
    pub r#type: SensorType,
}

impl_model_table!(db::TABLE_SENSOR, Sensor);

#[derive(Clone, Serialize)]
pub enum SensorType {
    #[serde(rename = "T")]
    Temperature,
    #[serde(rename = "P")]
    Pulse,
    #[serde(rename = "L")]
    Location,
    #[serde(rename = "R")]
    Respiration,
}

impl SensorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SensorType::Temperature => "T",
            SensorType::Pulse => "P",
            SensorType::Location => "L",
            SensorType::Respiration => "R",
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "T" => SensorType::Temperature,
            "P" => SensorType::Pulse,
            "L" => SensorType::Location,
            "R" => SensorType::Respiration,
            s => panic!("unsupported SensorType {}", s),
        }
    }
}

impl FromCqlVal<CqlValue> for SensorType {
    fn from_cql(cql_val: CqlValue) -> Result<Self, FromCqlValError> {
        Ok(SensorType::from_str(
            cql_val
                .as_text()
                .ok_or(FromCqlValError::BadCqlType)?
                .as_str(),
        ))
    }
}

impl Value for SensorType {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), ValueTooBig> {
        let bytes = self.as_str().as_bytes();
        buf.put_i32(bytes.len().try_into().map_err(|_| ValueTooBig)?);
        buf.put(bytes);

        Ok(())
    }
}

impl SensorType {
    fn random() -> Self {
        match rand::random::<usize>() % 4 {
            0 => SensorType::Temperature,
            1 => SensorType::Pulse,
            2 => SensorType::Location,
            3 => SensorType::Respiration,
            _ => unreachable!(),
        }
    }

    pub const fn len() -> usize {
        4
    }
}

impl Sensor {
    pub fn random(p: &Pet) -> Self {
        Self {
            pet_id: p.pet_id,
            sensor_id: Uuid::new_v4(),
            r#type: SensorType::random(),
        }
    }
}

pub fn random_sensor_data(sensor: &Sensor) -> f32 {
    let mut rng = rand::thread_rng();

    match sensor.r#type {
        SensorType::Temperature => 101.0 + rng.gen_range(0.0..10.0) - 4.0,
        SensorType::Pulse => 101.0 + rng.gen_range(0.0..40.0) - 20.0,
        SensorType::Respiration => 35.0 + rng.gen_range(0.0..5.0) - 2.0,
        SensorType::Location => 10.0 * rand::random::<f32>(),
    }
}

#[derive(Debug, ValueList, StructFieldNames, FieldNamesAsArray)]
pub struct Measure {
    pub sensor_id: Uuid,
    pub ts: Duration,
    pub value: f32,
}

impl_model_table!(db::TABLE_MEASUREMENT, Measure);

#[derive(
    Debug, ValueList, FromRow, Serialize, Default, Clone, StructFieldNames, FieldNamesAsArray,
)]
pub struct SensorAvg {
    pub sensor_id: Uuid,
    pub date: Date,
    pub hour: i32,
    pub value: f32,
}

impl_model_table!(db::TABLE_SENSOR_AVG, SensorAvg);

fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..len)
        .map(|_| rng.gen_range(b'a'..=b'z') as char)
        .collect()
}
