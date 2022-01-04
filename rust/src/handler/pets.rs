use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use scylla::{IntoTypedRows, Session};

use crate::db;
use crate::handler::{json_err, JsonError, UuidParam};
use crate::{ModelTable, Pet};

#[get("/owner/<id>/pets")]
pub async fn find_pets_by_owner_id(
    sess: &State<Session>,
    id: UuidParam,
) -> Result<Json<Vec<Pet>>, JsonError> {
    let pets = sess
        .query(
            format!(
                "SELECT {} FROM {} WHERE {} = ?",
                db::fields(Pet::FIELD_NAMES_AS_ARRAY),
                Pet::table(),
                Pet::FIELD_NAMES.owner_id,
            ),
            (id.0,),
        )
        .await
        .map_err(|err| json_err(Status::InternalServerError, err))?
        .rows
        .unwrap_or_default()
        .into_typed::<Pet>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| json_err(Status::InternalServerError, err))?;

    Ok(Json(pets))
}
