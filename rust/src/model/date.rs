use scylla::frame::response::cql_to_rust::{FromCqlVal, FromCqlValError};
use scylla::frame::response::result::CqlValue;
use scylla::frame::value::{Value, ValueTooBig};

use crate::duration::Duration;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Date(pub Duration);

impl Value for Date {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), ValueTooBig> {
        scylla::frame::value::Date(self.0 .0.num_days() as u32).serialize(buf)
    }
}

impl FromCqlVal<CqlValue> for Date {
    fn from_cql(cql_val: CqlValue) -> Result<Self, FromCqlValError> {
        Duration::from_cql(cql_val).map(Self)
    }
}
