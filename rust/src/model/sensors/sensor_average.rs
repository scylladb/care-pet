use charybdis::macros::scylla::FromRow;
use charybdis::types::Uuid;
use chrono::NaiveDate;
use scylla::ValueList;
use serde::Serialize;


#[derive(Clone, Debug, Serialize, ValueList, FromRow)]
pub struct SensorAvg {
    pub sensor_id: Uuid,
    pub date: NaiveDate,
    pub hour: i32,
    pub value: f32,
}