use std::collections::HashMap;

use chrono::{DateTime, Datelike, NaiveDateTime, NaiveTime, Timelike, Utc};
use log::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use scylla::{IntoTypedRows, Session};
use uuid::Uuid;

use crate::date::Date;
use crate::db::{self, TABLE_MEASUREMENT, TABLE_SENSOR_AVG};
use crate::duration::Duration;
use crate::handler::{json_err, DateParam, JsonError, UuidParam};
use crate::{Measure, SensorAvg};

#[get("/sensor/<id>/values/day/<date>")]
pub async fn find_sensor_avg_by_sensor_id_and_day(
    sess: &State<Session>,
    id: UuidParam,
    date: DateParam,
) -> Result<Json<Vec<f32>>, JsonError> {
    let date = date.0.to_date();
    if date.ordinal() > chrono::Utc::now().ordinal() {
        return Err(json_err(
            Status::BadRequest,
            anyhow::anyhow!("day cannot be in the future"),
        ));
    }

    let mut avg = sess
        .query(
            format!(
                "SELECT * FROM {} WHERE {} = ? AND {} = ?",
                TABLE_SENSOR_AVG,
                SensorAvg::FIELD_NAMES.sensor_id,
                SensorAvg::FIELD_NAMES.date,
            ),
            (id.0, date.date().naive_utc()),
        )
        .await
        .map_err(|err| json_err(Status::InternalServerError, err))?
        .rows
        .unwrap_or_default()
        .into_typed::<SensorAvg>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| json_err(Status::InternalServerError, err))?;

    if avg.len() < 24 {
        avg = aggregate(sess, id.0, date, avg)
            .await
            .ok_or_else(|| json_err(Status::InternalServerError, anyhow::anyhow!("aggregating")))?
    }

    Ok(Json(avg.into_iter().map(|avg| avg.value).collect()))
}

async fn aggregate(
    sess: &State<Session>,
    id: Uuid,
    date: DateTime<Utc>,
    avg: Vec<SensorAvg>,
) -> Option<Vec<SensorAvg>> {
    let now = Utc::now();

    let start_hour = avg.len();
    let start_date: DateTime<Utc> = DateTime::from_utc(
        NaiveDateTime::new(
            date.naive_utc().date(),
            NaiveTime::from_hms(start_hour as u32, 0, 0),
        ),
        Utc,
    );
    let end_date: DateTime<Utc> = DateTime::from_utc(
        NaiveDateTime::new(
            date.naive_utc().date(),
            NaiveTime::from_hms_milli(23, 59, 59, 0),
        ),
        Utc,
    );

    let prev_avg_size = avg.len();
    let data = load_data(sess, id, start_date, end_date).await?;
    let mut avg = group_by(avg, data, start_hour, date, now);
    save_aggregate(sess, id, &mut avg, prev_avg_size, start_date, date, now).await;

    Some(avg)
}

async fn load_data(
    sess: &State<Session>,
    id: Uuid,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Option<Vec<(Duration, f32)>> {
    sess.query(
        format!(
            "SELECT ts, value FROM {} WHERE {} = ? and {} >= ? and {} <= ?",
            TABLE_MEASUREMENT,
            Measure::FIELD_NAMES.sensor_id,
            Measure::FIELD_NAMES.ts,
            Measure::FIELD_NAMES.ts,
        ),
        (
            id,
            Duration::from_millis(start_date.timestamp_millis()),
            Duration::from_millis(end_date.timestamp_millis()),
        ),
    )
    .await
    .map_err(|err| error!("read sensor data query {:?}", err))
    .ok()?
    .rows
    .unwrap_or_default()
    .into_typed::<(Duration, f32)>()
    .collect::<Result<Vec<_>, _>>()
    .map_err(|err| error!("converting sensor data values {:?}", err))
    .ok()
}

fn group_by(
    avg: Vec<SensorAvg>,
    data: Vec<(Duration, f32)>,
    start_hour: usize,
    day: DateTime<Utc>,
    now: DateTime<Utc>,
) -> Vec<SensorAvg> {
    // if it's the same day, we can't aggregate current hour
    let same_date = now.ordinal() == day.ordinal();
    let last = now.hour();

    // aggregate data
    let mut ag: HashMap<i32, (f64, u64)> =
        data.iter()
            .fold(HashMap::default(), |mut acc, (ts, value)| {
                let hour = ts.to_date().hour();
                let (ag_value, ag_total) = acc.entry(hour as i32).or_default();
                *ag_value += *value as f64;
                *ag_total += 1;
                acc
            });

    // ensure data completeness
    for hour in start_hour..24 {
        if !same_date || hour <= last as usize {
            ag.entry(hour as i32).or_default();
        }
    }

    // fill the avg
    let mut avg = ag
        .iter()
        .map(|(hour, (value, total))| {
            let mut sa = SensorAvg {
                hour: *hour,
                ..Default::default()
            };
            if *total > 0 {
                sa.value = (value / *total as f64) as f32;
            }

            sa
        })
        .chain(avg)
        .collect::<Vec<_>>();

    avg.sort_by(|a, b| a.hour.cmp(&b.hour));
    avg
}

async fn save_aggregate(
    sess: &State<Session>,
    id: Uuid,
    avg: &mut [SensorAvg],
    prev_avg_size: usize,
    start_date: DateTime<Utc>,
    day: DateTime<Utc>,
    now: DateTime<Utc>,
) {
    let same_date = now.ordinal() == day.ordinal();
    let current = now.hour();

    let to_insert = avg
        .iter_mut()
        .skip(prev_avg_size)
        .filter(|avg| !same_date || avg.hour < current as i32);

    for mut avg in to_insert {
        avg.sensor_id = id;
        avg.date = Date(Duration::from_millis(start_date.timestamp_millis()));
        info!("inserting sensor aggregate {:?}", &avg);

        sess.query(
            format!(
                "INSERT INTO {} ({}) VALUES ({})",
                TABLE_SENSOR_AVG,
                db::fields(SensorAvg::FIELD_NAMES_AS_ARRAY),
                db::values::<{ SensorAvg::FIELD_NAMES_AS_ARRAY.len() }>(),
            ),
            avg.clone(),
        )
        .await
        .map_err(|err| error!("save sensor aggregate {:?}: {:?}", avg, err))
        .ok();
    }
}
