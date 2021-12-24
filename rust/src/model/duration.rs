use std::fmt::Formatter;
use std::time;

use chrono::{NaiveDateTime, NaiveTime, Utc};
use scylla::frame::response::result::CqlValue;
use scylla::frame::value::{Value, ValueTooBig};
use scylla::frame::{
    response::cql_to_rust::{FromCqlVal, FromCqlValError},
    value::Timestamp,
};
use serde::de::Error;
use serde::{Deserializer, Serializer};

#[derive(Debug, Clone)]
pub struct Duration(pub chrono::Duration);

impl Duration {
    pub fn now() -> Self {
        Self(chrono::Duration::milliseconds(
            chrono::Utc::now().timestamp_millis(),
        ))
    }

    pub fn from_seconds(secs: i64) -> Self {
        Self(chrono::Duration::seconds(secs))
    }

    pub fn from_millis(millis: i64) -> Self {
        Self(chrono::Duration::milliseconds(millis))
    }

    pub fn to_date(&self) -> chrono::DateTime<Utc> {
        chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp(self.0.num_seconds(), 0),
            Utc,
        )
    }

    pub fn format_rfc3339(&self) -> String {
        humantime::format_rfc3339(
            time::UNIX_EPOCH + time::Duration::from_millis(self.0.num_milliseconds() as u64),
        )
        .to_string()
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self(chrono::Duration::zero())
    }
}

impl Value for Duration {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), ValueTooBig> {
        Timestamp(self.0).serialize(buf)
    }
}

impl FromCqlVal<CqlValue> for Duration {
    fn from_cql(cql_val: CqlValue) -> Result<Self, FromCqlValError> {
        if let Some(d) = cql_val.as_date() {
            Ok(Self::from_millis(
                NaiveDateTime::new(d, NaiveTime::from_hms(0, 0, 0)).timestamp_millis(),
            ))
        } else {
            chrono::Duration::from_cql(cql_val).map(Self)
        }
    }
}

impl serde::ser::Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0.num_milliseconds())
    }
}

struct I64Visitor;

impl<'de> serde::de::Visitor<'de> for I64Visitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an i64 value")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v as i64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v as i64)
    }
}

impl<'de> serde::de::Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_i64(I64Visitor)
            .map(|millis| Self(chrono::Duration::milliseconds(millis)))
    }
}
