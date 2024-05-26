use actix_web::{get, HttpResponse, Responder};
use actix_web::web;
use serde_json::json;
use uuid::Uuid;

use crate::AppState;
use crate::controllers::SomeError;
use crate::repositories::pet_repository::PetRepository;

#[get("/owner/{owner_id}/pets")]
pub async fn find_pets_by_owner_id(
    data: web::Data<AppState>,
    owner_id: Option<web::Path<Uuid>>,
) -> actix_web::Result<impl Responder, SomeError> {

    let owner_id = owner_id.unwrap().into_inner();
    let pet_repository = PetRepository::new(data.session.clone()).await;

    let result = pet_repository.list_by_owner_id(owner_id, 10).await;

    match result {
        Ok(pets) => Ok(HttpResponse::Ok().json(pets)),
        Err(e) => Err(SomeError::InternalError(e)),
    }

    // let pets = sess
    //     .query(
    //         format!(
    //             "SELECT {} FROM {} WHERE {} = ?",
    //             db::fields(Pet::FIELD_NAMES_AS_ARRAY),
    //             Pet::table(),
    //             Pet::FIELD_NAMES.owner_id,
    //         ),
    //         (id.0,),
    //     )
    //     .await
    //     .map_err(|err| json_err(Status::InternalServerError, err))?
    //     .rows
    //     .unwrap_or_default()
    //     .into_typed::<Pet>()
    //     .collect::<Result<Vec<_>, _>>()
    //     .map_err(|err| json_err(Status::InternalServerError, err))?;


}
