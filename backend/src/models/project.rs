use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

fn validate_budget_min(value: &BigDecimal) -> Result<(), ValidationError> {
    use bigdecimal::FromPrimitive;
    let min = BigDecimal::from_f64(0.0).unwrap();
    if value < &min {
        return Err(ValidationError::new("budget must be non-negative"));
    }
    Ok(())
}

fn validate_dates(project: &ProjectCreate) -> Result<(), ValidationError> {
    if project.end_date <= project.start_date {
        return Err(ValidationError::new("end date must be after start date"));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Project {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "ERP System Implementation")]
    pub name: String,
    #[schema(example = "Implementation of a new ERP system for the company")]
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: ProjectStatus,
    #[schema(value_type = String, example = "150000.00")]
    pub budget: BigDecimal,
    pub client_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "project_status", rename_all = "snake_case")]
pub enum ProjectStatus {
    Planning,
    Development,
    Testing,
    Deployment,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ProjectCreate {
    #[validate(length(min = 1, max = 255, message = "name length must be between 1 and 255"))]
    pub name: String,
    #[schema(example = "Implementation of a new ERP system for the company")]
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    #[schema(value_type = String, example = "150000.00")]
    #[validate(custom(function = "validate_budget_min"))]
    pub budget: BigDecimal,
    pub client_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ProjectUpdate {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Option<ProjectStatus>,
    #[schema(value_type = String, example = "150000.00")]
    #[validate(custom(function = "validate_budget_min"))]
    pub budget: Option<BigDecimal>,
    pub client_id: Option<Uuid>,
}
