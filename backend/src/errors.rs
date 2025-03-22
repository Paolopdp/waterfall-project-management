use actix_web::{HttpResponse, ResponseError};
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use serde::Serialize;
use sqlx::Error as SqlxError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Internal server error")]
    InternalServerError,

    #[error("Forbidden")]
    Forbidden,

    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),

    #[error("Password hashing error: {0}")]
    PasswordHashError(#[from] BcryptError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl From<ValidationErrors> for ServiceError {
    fn from(err: ValidationErrors) -> Self {
        ServiceError::ValidationError(err.to_string())
    }
}

impl From<JwtError> for ServiceError {
    fn from(_: JwtError) -> Self {
        ServiceError::InternalServerError
    }
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

            ServiceError::InvalidCredentials => HttpResponse::Unauthorized().json(ErrorResponse {
                error: "Invalid credentials".to_string(),
            }),
            ServiceError::Forbidden => HttpResponse::Forbidden().json(ErrorResponse {
                error: self.to_string(),
            }),
            ServiceError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error occurred".to_string(),
                })
            }
            ServiceError::PasswordHashError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Password processing error occurred".to_string(),
                })
            }
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(ErrorResponse {
                error: message.clone(),
            }),
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
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
