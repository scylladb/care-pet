use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use scylla::{IntoTypedRows, Session};

use crate::db;
use crate::handler::{json_err, JsonError, UuidParam};
use crate::{ModelTable, Sensor};

#[get("/pet/<id>/sensors")]
pub async fn find_sensors_by_pet_id(
    sess: &State<Session>,
    id: UuidParam,
) -> Result<Json<Vec<Sensor>>, JsonError> {
    let pets = sess
        .query(
            format!(
                "SELECT {} FROM {} WHERE {} = ?",
                db::fields(Sensor::FIELD_NAMES_AS_ARRAY),
                Sensor::table(),
                Sensor::FIELD_NAMES.pet_id,
            ),
            (id.0,),
        )
        .await
        .map_err(|err| json_err(Status::InternalServerError, err))?
        .rows
        .unwrap_or_default()
        .into_typed::<Sensor>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| json_err(Status::InternalServerError, err))?;

    Ok(Json(pets))
}
