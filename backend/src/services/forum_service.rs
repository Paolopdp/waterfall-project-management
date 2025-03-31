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

        // If there are tags, add them
        if let Some(tag_ids) = thread.tag_ids {
            self.add_thread_tags(result.id, tag_ids, pool).await?;
        }

        // Get the complete thread with relations
        self.get_thread_with_relations(result.id, pool).await
    }

    pub async fn get_threads(&self, pool: &PgPool) -> Result<Vec<Thread>, ServiceError> {
        let threads = sqlx::query_as::<_, Thread>(
            r#"
            SELECT t.id, t.title, t.content, t.author_id, t.created_at, t.updated_at,
                COALESCE(json_agg(DISTINCT ft) FILTER (WHERE ft.id IS NOT NULL), '[]') AS tags,
                COALESCE(json_agg(DISTINCT a) FILTER (WHERE a.id IS NOT NULL), '[]') AS attachments
            FROM threads t
            LEFT JOIN thread_tags tt ON t.id = tt.thread_id
            LEFT JOIN forum_tags ft ON tt.tag_id = ft.id
            LEFT JOIN forum_attachments a ON t.id = a.thread_id
            GROUP BY t.id
            ORDER BY t.created_at DESC
            "#,
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
            r#"
            SELECT t.id, t.title, t.content, t.author_id, t.created_at, t.updated_at,
                COALESCE(json_agg(DISTINCT ft) FILTER (WHERE ft.id IS NOT NULL), '[]') AS tags,
                COALESCE(json_agg(DISTINCT a) FILTER (WHERE a.id IS NOT NULL), '[]') AS attachments
            FROM threads t
            LEFT JOIN thread_tags tt ON t.id = tt.thread_id
            LEFT JOIN forum_tags ft ON tt.tag_id = ft.id
            LEFT JOIN forum_attachments a ON t.id = a.thread_id
            "#,
        );

        let mut first_condition = true;

        if let Some(search) = params.query.as_ref() {
            query.push(if first_condition { " WHERE " } else { " AND " });
            first_condition = false;
            query.push("t.search_vector @@ plainto_tsquery(");
            query.push_bind(search);
            query.push(")");
        }

        if let Some(tags) = params.tags.as_ref() {
            if !tags.is_empty() {
                query.push(if first_condition { " WHERE " } else { " AND " });
                first_condition = false;
                query.push("tt.tag_id = ANY(");
                query.push_bind(tags.clone());
                query.push(")");
            }
        }

        if let Some(author) = params.author_id {
            query.push(if first_condition { " WHERE " } else { " AND " });
            first_condition = false;
            query.push("t.author_id = ");
            query.push_bind(author);
        }

        if let Some(from_date) = params.from_date {
            query.push(if first_condition { " WHERE " } else { " AND " });
            first_condition = false;
            query.push("t.created_at >= ");
            query.push_bind(from_date);
        }

        if let Some(to_date) = params.to_date {
            query.push(if first_condition { " WHERE " } else { " AND " });
            first_condition = false;
            query.push("t.created_at <= ");
            query.push_bind(to_date);
        }

        query.push(" GROUP BY t.id ");
        query.push(" ORDER BY t.created_at DESC ");
        query.push(" LIMIT ");
        query.push_bind(params.get_limit());
        query.push(" OFFSET ");
        query.push_bind(params.get_offset());

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
        let thread = sqlx::query_as::<_, Thread>(
            r#"
            SELECT t.id, t.title, t.content, t.author_id, t.created_at, t.updated_at,
                COALESCE(json_agg(DISTINCT ft) FILTER (WHERE ft.id IS NOT NULL), '[]') AS tags,
                COALESCE(json_agg(DISTINCT a) FILTER (WHERE a.id IS NOT NULL), '[]') AS attachments
            FROM threads t
            LEFT JOIN thread_tags tt ON t.id = tt.thread_id
            LEFT JOIN forum_tags ft ON tt.tag_id = ft.id
            LEFT JOIN forum_attachments a ON t.id = a.thread_id
            WHERE t.id = $1
            GROUP BY t.id
            "#,
        )
        .bind(thread_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ServiceError::NotFound("Thread not found".to_string()))?;

        Ok(thread)
    }
}
