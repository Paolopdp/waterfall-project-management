use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::models::project::{Project, ProjectCreate, ProjectStatus, ProjectUpdate};

pub struct ProjectService;

impl ProjectService {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Project>, ServiceError> {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id, name, description, start_date, end_date,
                status as "status: ProjectStatus", budget, client_id,
                created_at, updated_at
            FROM projects
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            ServiceError::DatabaseError(e)
        })?;

        Ok(projects)
    }

    pub async fn get_by_id(id: Uuid, pool: &PgPool) -> Result<Project, ServiceError> {
        let project = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id, name, description, start_date, end_date,
                status as "status: ProjectStatus", budget, client_id,
                created_at, updated_at
            FROM projects
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            ServiceError::DatabaseError(e)
        })?;

        project.ok_or(ServiceError::NotFound("Project not found".to_string()))
    }

    pub async fn create(
        new_project: ProjectCreate,
        pool: &PgPool,
    ) -> Result<Project, ServiceError> {
        let now = Utc::now();

        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (name, description, start_date, end_date, status, budget, client_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'planning', $5, $6, $7, $7)
            RETURNING id, name, description, start_date, end_date,
                      status as "status: ProjectStatus", budget, client_id,
                      created_at, updated_at
            "#,
            new_project.name,
            new_project.description,
            new_project.start_date,
            new_project.end_date,
            new_project.budget,
            new_project.client_id,
            now
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            ServiceError::DatabaseError(e)
        })?;

        Ok(project)
    }

    pub async fn update(
        id: Uuid,
        update: ProjectUpdate,
        pool: &PgPool,
    ) -> Result<Project, ServiceError> {
        // First, get the existing project to make sure it exists
        let existing = Self::get_by_id(id, pool).await?;

        let now = Utc::now();

        // Update only the fields that were provided
        let name = update.name.unwrap_or(existing.name);
        let description = update.description.or(existing.description);
        let start_date = update.start_date.unwrap_or(existing.start_date);
        let end_date = update.end_date.unwrap_or(existing.end_date);
        let status = update.status.unwrap_or(existing.status);
        let budget = update.budget.unwrap_or(existing.budget);
        let client_id = update.client_id.or(existing.client_id);

        let updated_project = sqlx::query_as!(
            Project,
            r#"
            UPDATE projects
            SET name = $1, description = $2, start_date = $3, end_date = $4,
                status = $5, budget = $6, client_id = $7, updated_at = $8
            WHERE id = $9
            RETURNING id, name, description, start_date, end_date,
                      status as "status: ProjectStatus", budget, client_id,
                      created_at, updated_at
            "#,
            name,
            description,
            start_date,
            end_date,
            status as ProjectStatus,
            budget,
            client_id,
            now,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            ServiceError::DatabaseError(e)
        })?;

        Ok(updated_project)
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<(), ServiceError> {
        let result = sqlx::query!("DELETE FROM projects WHERE id = $1", id)
            .execute(pool)
            .await
            .map_err(|e| {
                log::error!("Database error: {:?}", e);
                ServiceError::DatabaseError(e)
            })?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::NotFound("Project not found".to_string()));
        }

        Ok(())
    }
}
