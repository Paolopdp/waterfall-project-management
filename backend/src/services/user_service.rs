use crate::errors::ServiceError;
use crate::models::user::{User, UserCreate, UserUpdate};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn create(user: UserCreate, pool: &PgPool) -> Result<User, ServiceError> {
        let password_hash = hash(user.password.as_bytes(), DEFAULT_COST)?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash, full_name, role)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, password_hash, full_name, role as "role: _", created_at, updated_at
            "#,
            user.email,
            password_hash,
            user.full_name,
            user.role as _
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_id(id: Uuid, pool: &PgPool) -> Result<User, ServiceError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, full_name, role as "role: _", created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ServiceError::NotFound("User not found".into()))?;

        Ok(user)
    }

    pub async fn get_by_email(email: &str, pool: &PgPool) -> Result<User, ServiceError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, full_name, role as "role: _", created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ServiceError::NotFound("User not found".into()))?;

        Ok(user)
    }

    pub async fn update(id: Uuid, user: UserUpdate, pool: &PgPool) -> Result<User, ServiceError> {
        let current_user = Self::get_by_id(id, pool).await?;

        let password_hash = if let Some(password) = user.password {
            hash(password.as_bytes(), DEFAULT_COST)?
        } else {
            current_user.password_hash
        };

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET
                email = COALESCE($1, email),
                password_hash = $2,
                full_name = COALESCE($3, full_name),
                role = COALESCE($4, role),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $5
            RETURNING id, email, password_hash, full_name, role as "role: _", created_at, updated_at
            "#,
            user.email,
            password_hash,
            user.full_name,
            user.role as _,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<(), ServiceError> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::NotFound("User not found".into()));
        }

        Ok(())
    }

    pub async fn list(pool: &PgPool) -> Result<Vec<User>, ServiceError> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, full_name, role as "role: _", created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}
