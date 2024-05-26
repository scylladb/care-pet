use actix_web::{get, HttpResponse, Responder, web};
use uuid::Uuid;

use crate::repositories::sensor_repository::SensorRepository;

use crate::AppState;
use crate::controllers::SomeError;
use crate::controllers::SomeError::InternalError;

#[get("/pet/{pet_id}/sensors")]
pub async fn find_sensors_by_pet_id(
    data: web::Data<AppState>,
    pet_id: Option<web::Path<Uuid>>,
) -> actix_web::Result<impl Responder, SomeError> {
    let sensor_repository = SensorRepository::new(data.session.clone()).await;
    let pet_id = pet_id.unwrap().into_inner();

    let sensors = sensor_repository.list_by_pet(pet_id, 10).await;

    match sensors {
        Ok(sensors) => Ok(HttpResponse::Ok().json(sensors)),
        Err(e) => Err(InternalError(e))
    }
}
