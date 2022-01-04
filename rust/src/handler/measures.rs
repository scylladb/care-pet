use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use scylla::{IntoTypedRows, Session};

use crate::handler::{json_err, DateTimeParam, JsonError, UuidParam};
use crate::{Measure, ModelTable};

#[get("/sensor/<id>/values?<from>&<to>")]
pub async fn find_sensor_data_by_sensor_id_and_time_range(
    session: &State<Session>,
    id: UuidParam,
    from: DateTimeParam,
    to: DateTimeParam,
) -> Result<Json<Vec<f32>>, JsonError> {
    let rows = session
        .query(
            format!(
                "SELECT {} FROM {} WHERE {} = ? and {} >= ? and {} <= ?",
                Measure::FIELD_NAMES.value,
                Measure::table(),
                Measure::FIELD_NAMES.sensor_id,
                Measure::FIELD_NAMES.ts,
                Measure::FIELD_NAMES.ts,
            ),
            (id.0, from.0, to.0),
        )
        .await
        .map_err(|err| json_err(Status::InternalServerError, err))?
        .rows
        .unwrap_or_default()
        .into_typed::<(f32,)>();

    let values = rows
        .map(|v| v.map(|v| v.0))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| json_err(Status::InternalServerError, err))?;

    Ok(Json(values))
}
