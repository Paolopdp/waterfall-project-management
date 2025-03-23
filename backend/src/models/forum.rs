use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Decode};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, sqlx::Type)]
pub struct ForumTag {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate, sqlx::Type)]
pub struct ForumAttachment {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateThreadRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Title must be between 1 and 100 characters"
    ))]
    pub title: String,
    #[validate(length(
        min = 1,
        max = 5000,
        message = "Content must be between 1 and 5000 characters"
    ))]
    pub content: String,
    pub tag_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateReplyRequest {
    #[validate(length(
        min = 1,
        max = 5000,
        message = "Content must be between 1 and 5000 characters"
    ))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTagRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Tag name must be between 1 and 50 characters"
    ))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Thread {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Option<Vec<ForumTag>>,
    pub attachments: Option<Vec<ForumAttachment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub content: String,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreadSearchParams {
    pub query: Option<String>,
    pub tags: Option<Vec<Uuid>>,
    pub author_id: Option<Uuid>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// Implementation for search params
impl ThreadSearchParams {
    pub fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }

    pub fn get_offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}
