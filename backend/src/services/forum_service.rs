use crate::errors::ServiceError;
use crate::models::forum::{
    CreateReplyRequest, CreateThreadRequest, ForumAttachment, ForumTag, Reply, Thread,
    ThreadSearchParams,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub struct ForumService {
    pool: PgPool,
}

impl ForumService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_thread(
        &self,
        author_id: Uuid,
        thread: CreateThreadRequest,
        pool: &PgPool,
    ) -> Result<Thread, ServiceError> {
        thread.validate()?;

        let result = sqlx::query!(
            r#"
            INSERT INTO threads (title, content, author_id, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            RETURNING
                id, title, content, author_id, created_at, updated_at
            "#,
            thread.title,
            thread.content,
            author_id
        )
        .fetch_one(pool)
        .await?;
        // Create a Thread with empty tags and attachments
        let thread = Thread {
            id: result.id,
            title: result.title,
            content: result.content,
            author_id: result.author_id,
            created_at: result.created_at,
            updated_at: result.updated_at,
            tags: None,
            attachments: None,
        };

        Ok(thread)
    }

    pub async fn get_threads(&self, pool: &PgPool) -> Result<Vec<Thread>, ServiceError> {
        let threads = sqlx::query_as!(
            Thread,
            r#"
            SELECT t.id, t.title, t.content, t.author_id, t.created_at, t.updated_at,
                COALESCE(NULLIF(ARRAY_AGG((ft.id, ft.name, ft.description, ft.created_at)),'{NULL}'), '{}') as "tags: Vec<ForumTag>",
                  COALESCE(NULLIF( ARRAY_AGG((a.id, a.filename, a.content_type, a.size_bytes, a.uploaded_by, a.created_at)),'{NULL}'), '{}') as "attachments: Vec<ForumAttachment>"
            FROM threads t LEFT JOIN thread_tags tt ON t.id = tt.thread_id
            LEFT JOIN forum_tags ft ON tt.tag_id = ft.id
            LEFT JOIN forum_attachments a ON t.id = a.thread_id
            group by t.id
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(threads)
    }

    pub async fn get_thread_replies(
        &self,
        thread_id: Uuid,
        pool: &PgPool,
    ) -> Result<Vec<Reply>, ServiceError> {
        let replies = sqlx::query_as!(
            Reply,
            r#"
            SELECT id, thread_id, content, author_id, created_at, updated_at
            FROM replies
            WHERE thread_id = $1
            ORDER BY created_at ASC
            "#,
            thread_id
        )
        .fetch_all(pool)
        .await?;

        Ok(replies)
    }

    pub async fn create_reply(
        &self,
        author_id: Uuid,
        thread_id: Uuid,
        reply: CreateReplyRequest,
        pool: &PgPool,
    ) -> Result<Reply, ServiceError> {
        reply.validate()?;

        // Check if thread exists
        let thread_exists = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1)",
            thread_id
        )
        .fetch_one(pool)
        .await?
        .exists
        .unwrap_or(false);

        if !thread_exists {
            return Err(ServiceError::NotFound("Thread not found".to_string()));
        }

        let result = sqlx::query_as!(
            Reply,
            r#"
            INSERT INTO replies (thread_id, content, author_id, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            RETURNING id, thread_id, content, author_id, created_at, updated_at
            "#,
            thread_id,
            reply.content,
            author_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn search_threads(
        &self,
        params: ThreadSearchParams,
        pool: &PgPool,
    ) -> Result<Vec<Thread>, ServiceError> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT DISTINCT t.id, t.title, t.content, t.author_id, t.created_at, t.updated_at
             FROM threads t
             LEFT JOIN thread_tags tt ON t.id = tt.thread_id
             WHERE 1=1",
        );

        // Before using params, take a reference to query
        let query_str = params.query.as_ref();
        if let Some(search) = query_str {
            query
                .push(" AND t.search_vector @@ plainto_tsquery(")
                .push_bind(search)
                .push(")");
        }

        if let Some(tags) = &params.tags {
            query
                .push(" AND tt.tag_id = ANY(")
                .push_bind(tags.clone())
                .push("::uuid[])");
        }

        if let Some(author) = params.author_id {
            query.push(" AND t.author_id = ").push_bind(author);
        }

        query
            .push(" ORDER BY t.created_at DESC LIMIT ")
            .push_bind(params.get_limit())
            .push(" OFFSET ")
            .push_bind(params.get_offset());

        let threads = query.build_query_as::<Thread>().fetch_all(pool).await?;

        Ok(threads)
    }

    pub async fn add_thread_tags(
        &self,
        thread_id: Uuid,
        tag_ids: Vec<Uuid>,
        pool: &PgPool,
    ) -> Result<(), ServiceError> {
        for tag_id in tag_ids {
            sqlx::query!(
                "INSERT INTO thread_tags (thread_id, tag_id) VALUES ($1, $2)
                 ON CONFLICT DO NOTHING",
                thread_id,
                tag_id
            )
            .execute(pool)
            .await?;
        }
        Ok(())
    }

    pub async fn create_tag(
        &self,
        name: String,
        description: Option<String>,
        pool: &PgPool,
    ) -> Result<ForumTag, ServiceError> {
        let tag = sqlx::query_as!(
            ForumTag,
            r#"
            INSERT INTO forum_tags (name, description)
            VALUES ($1, $2)
            RETURNING id, name, description, created_at
            "#,
            name,
            description
        )
        .fetch_one(pool)
        .await?;

        Ok(tag)
    }

    pub async fn get_thread_with_relations(
        &self,
        thread_id: Uuid,
        pool: &PgPool,
    ) -> Result<Thread, ServiceError> {
        let thread = sqlx::query_as!(
            Thread,
            r#"
              SELECT t.id, t.title, t.content, t.author_id, t.created_at, t.updated_at,
                COALESCE(NULLIF(ARRAY_AGG((ft.id, ft.name, ft.description, ft.created_at)),'{NULL}'), '{}') as "tags: Vec<ForumTag>",
                  COALESCE(NULLIF( ARRAY_AGG((a.id, a.filename, a.content_type, a.size_bytes, a.uploaded_by, a.created_at)),'{NULL}'), '{}') as "attachments: Vec<ForumAttachment>"
            FROM threads t LEFT JOIN thread_tags tt ON t.id = tt.thread_id
            LEFT JOIN forum_tags ft ON tt.tag_id = ft.id
            LEFT JOIN forum_attachments a ON t.id = a.thread_id
            WHERE t.id = $1
            GROUP BY t.id
            "#,
            thread_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServiceError::NotFound("Thread not found".to_string()))?;

        Ok(thread)
    }
}
