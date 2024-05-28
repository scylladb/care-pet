use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use serde_json::json;
use thiserror::Error;
pub mod sensors_controller;
pub mod owner_controller;
pub mod pets_controller;


#[derive(Error, Debug)]
pub enum SomeError {
    #[error("{0}")]
    InternalError(#[from] anyhow::Error)
}

impl ResponseError for SomeError {
    fn status_code(&self) -> StatusCode {
        match *self {
            SomeError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            SomeError::InternalError(e) => {
                HttpResponse::InternalServerError().json(json!({
                    "message": e.to_string()
                }))
            }
        }
    }
}