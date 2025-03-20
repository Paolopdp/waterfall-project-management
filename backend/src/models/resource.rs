use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    pub skills: Vec<String>,
    pub availability: f32, // percentage
    pub hourly_rate: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
