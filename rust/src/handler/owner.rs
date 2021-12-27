use anyhow::anyhow;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use scylla::Session;

use crate::db::TABLE_OWNER;
use crate::handler::{json_err, JsonError, UuidParam};
use crate::Owner;

#[get("/owner/<id>")]
pub async fn find_owner_by_id(
    sess: &State<Session>,
    id: UuidParam,
) -> Result<Json<Owner>, JsonError> {
    let results = sess
        .query(
            format!(
                "SELECT * FROM {} WHERE {} = ?",
                TABLE_OWNER,
                Owner::FIELD_NAMES.owner_id,
            ),
            (id.0,),
        )
        .await
        .map_err(|err| json_err(Status::InternalServerError, err))?
        .rows
        .unwrap_or_default();

    if results.is_empty() {
        return Err(json_err(
            Status::NotFound,
            anyhow!("owner {} not found", id.0),
        ));
    }

    let owner = results
        .into_iter()
        .next()
        .unwrap()
        .into_typed::<Owner>()
        .map_err(|err| json_err(Status::InternalServerError, err))?;

    Ok(Json(owner))
}
