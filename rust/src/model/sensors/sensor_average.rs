use charybdis::types::Uuid;
use rocket::time::Date;

pub struct SensorAvg {
    pub sensor_id: Uuid,
    pub date: Date,
    pub hour: i32,
    pub value: f32,
}