use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: TaskStatus,
    pub assigned_to: Vec<Uuid>,
    pub dependencies: Vec<Uuid>,
    #[schema(value_type = String, example = "150000.00")]
    pub progress: BigDecimal, // percentage
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq)]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct TaskCreate {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub project_id: Uuid,
    pub assigned_to: Option<Uuid>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub dependencies: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, Default)]
pub struct TaskUpdate {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    #[schema(value_type = TaskStatus)]
    pub status: Option<TaskStatus>,
    #[schema(value_type = String, example = "150000.00")]
    pub progress: Option<BigDecimal>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub dependencies: Option<Vec<Uuid>>,
}
