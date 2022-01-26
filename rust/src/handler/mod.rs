pub mod avg;
pub mod measures;
pub mod owner;
pub mod pets;
pub mod sensors;

use std::str::FromStr;

use chrono::{NaiveDateTime, NaiveTime};
use rocket::form::{FromFormField, ValueField};
use rocket::http::Status;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use uuid::Uuid;

use crate::duration::Duration;
use crate::result::Error;

#[derive(Serialize)]
pub struct JsonErrorData {
    error: String,
}

pub type JsonError = (Status, Json<JsonErrorData>);

pub fn json_err<E: Into<Error>>(status: Status, err: E) -> JsonError {
    (
        status,
        Json(JsonErrorData {
            error: format!("{:?}", err.into()),
        }),
    )
}

#[derive(Debug)]
pub struct UuidParam(pub Uuid);

impl<'a> FromParam<'a> for UuidParam {
    type Error = Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Uuid::from_str(param).map(Self).map_err(From::from)
    }
}

#[derive(Debug)]
pub struct DateTimeParam(pub Duration);

impl<'a> FromFormField<'a> for DateTimeParam {
    fn from_value(field: ValueField<'a>) -> rocket::form::Result<Self> {
        chrono::DateTime::parse_from_rfc3339(field.value)
            .map(|dt| Self(Duration::from_millis(dt.timestamp_millis())))
            .map_err(|_| field.unexpected().into())
    }
}

impl<'a> FromParam<'a> for DateTimeParam {
    type Error = Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        chrono::DateTime::parse_from_rfc3339(param)
            .map(|dt| Self(Duration::from_millis(dt.timestamp_millis())))
            .map_err(From::from)
    }
}

#[derive(Debug)]
pub struct DateParam(pub Duration);

impl<'a> FromParam<'a> for DateParam {
    type Error = Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        chrono::NaiveDate::parse_from_str(param, "%Y-%m-%d")
            .map(|d| {
                Self(Duration::from_millis(
                    NaiveDateTime::new(d, NaiveTime::from_hms(0, 0, 0)).timestamp_millis(),
                ))
            })
            .map_err(From::from)
    }
}
