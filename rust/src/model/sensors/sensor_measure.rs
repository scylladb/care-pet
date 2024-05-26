use charybdis::macros::scylla::FromRow;
use chrono::{DateTime, Utc};
use scylla::ValueList;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, ValueList, FromRow, Serialize)]
pub struct Measure {
    pub sensor_id: Uuid,
    pub ts: DateTime<Utc>,
    pub value: f32,
}
