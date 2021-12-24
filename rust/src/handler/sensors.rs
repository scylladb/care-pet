use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use scylla::{IntoTypedRows, Session};

use crate::db::TABLE_SENSOR;
use crate::handler::{json_err, JsonError, UuidParam};
use crate::{Pet, Sensor};

#[get("/pet/<id>/sensors")]
pub async fn find_sensors_by_pet_id(
    sess: &State<Session>,
    id: UuidParam,
) -> Result<Json<Vec<Sensor>>, JsonError> {
    let pets = sess
        .query(
            format!(
                "SELECT * FROM {} WHERE {} = ?",
                TABLE_SENSOR,
                Pet::FIELD_NAMES.pet_id,
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
