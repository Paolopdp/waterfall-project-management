use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Internal server error")]
    InternalServerError,

    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: self.to_string(),
                })
            }
            ServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error occurred".to_string(),
                })
            }
            ServiceError::NotFound(ref message) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: message.clone(),
                })
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: message.clone(),
                })
            }
            ServiceError::Unauthorized(ref message) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: message.clone(),
                })
            }
            ServiceError::ValidationError(ref message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: message.clone(),
                })
            }
        }
    }
}
