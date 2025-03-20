use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, PartialEq)]
pub struct User {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    pub password_hash: String,
    #[schema(example = "John Doe")]
    pub full_name: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    ProjectManager,
    Developer,
    QaEngineer,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserCreate {
    #[validate(email)]
    #[schema(example = "john.doe@example.com")]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "John Doe")]
    pub full_name: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserUpdate {
    #[validate(email)]
    #[schema(example = "john.doe@example.com")]
    pub email: Option<String>,
    #[validate(length(min = 8))]
    pub password: Option<String>,
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "John Doe")]
    pub full_name: Option<String>,
    pub role: Option<UserRole>,
}
