use actix_web::{get, HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use crate::http::controllers::SomeError;
use crate::http::controllers::SomeError::InternalError;
use crate::repositories::sensor_repository::SensorRepository;

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


#[derive(Deserialize, Debug, Clone)]
struct DateRangeQuery {
    from: String,
    to: String,
}

#[get("/sensors/{sensor_id}/values")]
pub async fn find_sensor_data_by_sensor_id_and_time_range(
    data: web::Data<AppState>,
    sensor_id: Option<web::Path<Uuid>>,
    payload: web::Query<DateRangeQuery>,
) -> actix_web::Result<impl Responder, SomeError> {
    let sensor_repository = SensorRepository::new(data.session.clone()).await;
    let sensor_id = sensor_id.unwrap().into_inner();

    let sensors = sensor_repository.list_pet_sensor_data_by_range(
        sensor_id,
        payload.from.as_str(),
        payload.to.as_str(),
    ).await;

    match sensors {
        Ok(sensors) => Ok(HttpResponse::Ok().json(sensors)),
        Err(e) => Err(InternalError(e))
    }
}

#[get("/sensors/{sensor_id}/values/day/{date}")]
pub async fn find_sensor_avg_by_sensor_id_and_day(
    data: web::Data<AppState>,
    path: web::Path<(Uuid, String)>,
) -> actix_web::Result<impl Responder, SomeError> {
    let (sensor_id, date) = path.into_inner();
    let sensor_repository = SensorRepository::new(data.session.clone()).await;

    let result = sensor_repository.find_sensor_avg_by_sensor_id_and_day(sensor_id, date.as_str()).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(InternalError(e))
    }
}
