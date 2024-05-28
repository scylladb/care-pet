use charybdis::macros::scylla::{FromRow, SerializeRow};
use charybdis::types::Uuid;
use chrono::NaiveDate;
use serde::Serialize;


#[derive(Clone, Debug, Serialize, SerializeRow, FromRow)]
pub struct SensorAvg {
    pub sensor_id: Uuid,
    pub date: NaiveDate,
    pub hour: i32,
    pub value: f32,
}