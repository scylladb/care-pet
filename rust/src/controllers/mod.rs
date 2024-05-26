use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use charybdis::errors::CharybdisError;
use serde_json::json;
use thiserror::Error;

// pub mod avg;
// pub mod measures;
pub mod sensors_controller;
pub mod owner_controller;
pub mod pets_controller;


#[derive(Error, Debug)]
pub enum SomeError {
    #[error("Charybdis error: {0}")]
    CharybdisError(#[from] CharybdisError),
    #[error("{0}")]
    InternalError(#[from] anyhow::Error)
}

impl ResponseError for SomeError {
    fn status_code(&self) -> StatusCode {
        match *self {
            SomeError::CharybdisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SomeError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            SomeError::CharybdisError(e) => match e {
                CharybdisError::NotFoundError(e) => HttpResponse::NotFound().json(json!({
                    "message": e.to_string()
                })),
                _ => HttpResponse::InternalServerError().json(json!({
                    "message": "Internal Server Error"
                }))
            },
            SomeError::InternalError(e) => {
                HttpResponse::InternalServerError().json(json!({
                    "message": e.to_string()
                }))
            }
        }
    }
}