use crate::errors::ServiceError;
use crate::models::task::{Task, TaskCreate, TaskStatus, TaskUpdate};
use sqlx::PgPool;
use uuid::Uuid;

pub struct TaskService;

impl TaskService {
    pub async fn get_all(db: &PgPool) -> Result<Vec<Task>, ServiceError> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id, name, description, project_id, assigned_to,
                status as "status: TaskStatus", progress,
                start_date, end_date, dependencies,
                created_at, updated_at
            FROM tasks
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(db)
        .await?;

        Ok(tasks)
    }

    pub async fn get_by_id(id: Uuid, db: &PgPool) -> Result<Task, ServiceError> {
        let task = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id, name, description, project_id, assigned_to,
                status as "status: TaskStatus", progress,
                start_date, end_date, dependencies,
                created_at, updated_at
            FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?
        .ok_or(ServiceError::NotFound("Task not found".into()))?;

        Ok(task)
    }

    pub async fn create(task: TaskCreate, db: &PgPool) -> Result<Task, ServiceError> {
        let task = sqlx::query_as!(
            Task,
            r#"
            INSERT INTO tasks (
                name, description, project_id, assigned_to,
                status, progress, start_date, end_date, dependencies
            )
            VALUES ($1, $2, $3, $4, 'pending', 0, $5, $6, $7)
            RETURNING
                id, name, description, project_id, assigned_to,
                status as "status: TaskStatus", progress,
                start_date, end_date, dependencies,
                created_at, updated_at
            "#,
            task.name,
            task.description,
            task.project_id,
            &task.assigned_to.map(|id| vec![id]).unwrap_or_default(),
            task.start_date,
            task.end_date,
            &task.dependencies
        )
        .fetch_one(db)
        .await?;

        Ok(task)
    }

    pub async fn update(id: Uuid, task: TaskUpdate, db: &PgPool) -> Result<Task, ServiceError> {
        // let current = Self::get_by_id(id, db).await?;

        let task = sqlx::query_as!(
            Task,
            r#"
            UPDATE tasks
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                assigned_to = COALESCE($3, assigned_to),
                status = COALESCE($4, status),
                progress = COALESCE($5, progress),
                start_date = COALESCE($6, start_date),
                end_date = COALESCE($7, end_date),
                dependencies = COALESCE($8, dependencies),
                updated_at = NOW()
            WHERE id = $9
            RETURNING
                id, name, description, project_id, assigned_to,
                status as "status: TaskStatus", progress,
                start_date, end_date, dependencies,
                created_at, updated_at
            "#,
            task.name,
            task.description,
            &task.assigned_to.map(|id| vec![id]).unwrap_or_default(),
            task.status as Option<TaskStatus>,
            task.progress,
            task.start_date,
            task.end_date,
            &task.dependencies.unwrap_or_default(),
            id
        )
        .fetch_one(db)
        .await?;

        Ok(task)
    }

    pub async fn get_by_project(project_id: Uuid, db: &PgPool) -> Result<Vec<Task>, ServiceError> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id, name, description, project_id, assigned_to,
                status as "status: TaskStatus", progress,
                start_date, end_date, dependencies,
                created_at, updated_at
            FROM tasks
            WHERE project_id = $1
            ORDER BY created_at DESC
            "#,
            project_id
        )
        .fetch_all(db)
        .await?;

        Ok(tasks)
    }

    pub async fn get_by_resource(
        resource_id: Uuid,
        db: &PgPool,
    ) -> Result<Vec<Task>, ServiceError> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT
                id, name, description, project_id, assigned_to,
                status as "status: TaskStatus", progress,
                start_date, end_date, dependencies,
                created_at, updated_at
            FROM tasks
            WHERE assigned_to = $1
            ORDER BY created_at DESC
            "#,
            &vec![resource_id]
        )
        .fetch_all(db)
        .await?;

        Ok(tasks)
    }

    pub async fn delete(id: Uuid, db: &PgPool) -> Result<(), ServiceError> {
        let result = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ServiceError::NotFound("Task not found".into()));
        }

        Ok(())
    }
}
