use scylla::_macro_internal::CqlValue;
use scylla::BufMut;
use scylla::cql_to_rust::{FromCqlVal, FromCqlValError};
use scylla::frame::value::{Value, ValueTooBig};
use serde::Serialize;

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

impl FromCqlVal<CqlValue> for SensorType {
    fn from_cql(cql_val: CqlValue) -> Result<Self, FromCqlValError> {
        Ok(SensorType::from_str(cql_val
            .as_text()
            .ok_or(FromCqlValError::BadCqlType)?
            .as_str()))
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

impl Default for SensorType {
    fn default() -> Self {
        SensorType::Temperature
    }
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
    pub fn random() -> Self {
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