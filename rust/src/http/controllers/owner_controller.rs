use actix_web::{get, HttpResponse, Responder};
use actix_web::web;
use uuid::Uuid;

use crate::AppState;
use crate::http::controllers::SomeError;
use crate::repositories::owner_repository::OwnerRepository;

#[get("/owner/{owner_id}")]
pub async fn index(
    data: web::Data<AppState>,
    owner_id: Option<web::Path<Uuid>>,
) -> actix_web::Result<impl Responder, SomeError> {
    let owner_repository = OwnerRepository::new(
        data.session.clone()
    ).await;

    let id = owner_id.unwrap();
    let owner = owner_repository.find(id.into_inner()).await;

    match owner {
        Ok(owner) => Ok(HttpResponse::Ok().json(owner)),
        Err(e) => Err(SomeError::InternalError(e)),
    }
}
