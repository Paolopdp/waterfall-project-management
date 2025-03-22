use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone, Copy)]
#[sqlx(type_name = "lifecycle_phase", rename_all = "snake_case")]
pub enum LifecyclePhase {
    Proposal,
    Requirements,
    Design,
    Implementation,
    Testing,
    Deployment,
    Maintenance,
    Closed,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PhaseTransition {
    pub project_id: Uuid,
    pub phase: LifecyclePhase,
    #[validate(length(min = 1, message = "Description is required"))]
    pub description: String,
    pub attachments: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhaseDetails {
    pub id: Uuid,
    pub project_id: Uuid,
    pub phase: LifecyclePhase,
    pub description: String,
    pub attachments: Option<Vec<String>>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
