use std::str::FromStr;
use std::sync::Arc;
use scylla::Session;
use uuid::Uuid;
use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use crate::model::sensors::sensor::Sensor;
use crate::model::sensors::sensor_average::SensorAvg;
use crate::model::sensors::sensor_measure::Measure;

pub struct SensorRepository {
    session: Arc<Session>,
}

impl SensorRepository {
    pub async fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    pub async fn list_by_pet(&self, id: Uuid, per_page: i32) -> Result<Vec<Sensor>> {
        let query = "\
        SELECT \
            pet_id,
            sensor_id,
            type
         FROM
            sensors
         WHERE
            pet_id = ?
        ";

        let mut prepared = self.session.prepare(query).await?;
        prepared.set_page_size(per_page);

        let result = self.session.execute(&prepared, (id, )).await?;

        let sensors = result.rows_typed::<Sensor>()?.collect::<Result<Vec<_>, _>>()?;
        if sensors.len() == 0 {
            return Err(anyhow!("Sensors not found"));
        }

        Ok(sensors)
    }

    pub async fn list_pet_sensor_data_by_range(&self, id: Uuid, from: &str, to: &str) -> Result<Vec<Measure>> {
        let from_naive = NaiveDate::from_str(from).unwrap();
        let from = DateTime::<Utc>::from_utc(from_naive.and_hms(0, 0, 0), Utc);

        let to_naive = NaiveDate::from_str(to).unwrap();
        let to = DateTime::<Utc>::from_utc(to_naive.and_hms(23, 59, 59), Utc);

        let query = "\
            SELECT \
                sensor_id,
                ts,
                value
             FROM
                measurement
             WHERE
                sensor_id = ? AND
                ts >= ? AND
                ts <= ?
        ";

        let prepared = self.session.prepare(query).await?;
        let result = self.
            session.execute(&prepared, (id, from.to_utc(), to.to_utc(), )).await?;

        let values = result.rows_typed::<Measure>()?.collect::<Result<Vec<_>, _>>()?;
        if values.len() == 0 {
            return Err(anyhow!("Sensor data not found"));
        }

        Ok(values)
    }

    pub async fn find_sensor_avg_by_sensor_id_and_day(&self, id: Uuid, date: &str) -> Result<Vec<SensorAvg>> {
        let date = NaiveDate::from_str(date).unwrap();

        let query = "\
            SELECT \
                sensor_id,
                date,
                hour,
                value
             FROM
                sensor_avg
             WHERE
                sensor_id = ? AND
                date = ?
        ";

        let prepared = self.session.prepare(query).await?;
        let result = self.
            session.execute(&prepared, (id, date)).await?;

        let values = result.rows_typed::<SensorAvg>()?.collect::<Result<Vec<_>, _>>()?;
        if values.len() == 0 {
            return Err(anyhow!("Sensor data not found"));
        }
        Ok(values)
    }
}