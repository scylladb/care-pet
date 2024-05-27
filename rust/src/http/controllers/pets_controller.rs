use actix_web::{get, HttpResponse, Responder};
use actix_web::web;
use uuid::Uuid;

use crate::AppState;
use crate::http::controllers::SomeError;
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
}
