use crate::errors::ServiceError;
use crate::models::task::{TaskCreate, TaskUpdate};
use crate::services::task_service::TaskService;
use actix_web::{delete, get, post, put, web, HttpResponse};
use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .service(get_tasks)
            .service(get_task)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
            .service(get_project_tasks)
            .service(get_resource_tasks)
            .service(update_task_progress),
    );
}

#[get("")]
async fn get_tasks(db: web::Data<PgPool>) -> Result<HttpResponse, ServiceError> {
    let tasks = TaskService::get_all(&db).await?;
    Ok(HttpResponse::Ok().json(tasks))
}

#[get("/{id}")]
async fn get_task(
    id: web::Path<Uuid>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let task = TaskService::get_by_id(id.into_inner(), &db).await?;
    Ok(HttpResponse::Ok().json(task))
}

#[post("")]
async fn create_task(
    task: web::Json<TaskCreate>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    task.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;

    let task = TaskService::create(task.into_inner(), &db).await?;
    Ok(HttpResponse::Created().json(task))
}

#[put("/{id}")]
async fn update_task(
    id: web::Path<Uuid>,
    task: web::Json<TaskUpdate>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    task.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;

    let task = TaskService::update(id.into_inner(), task.into_inner(), &db).await?;
    Ok(HttpResponse::Ok().json(task))
}

#[delete("/{id}")]
async fn delete_task(
    id: web::Path<Uuid>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    sqlx::query!("DELETE FROM tasks WHERE id = $1", id.into_inner())
        .execute(&**db)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/project/{project_id}")]
async fn get_project_tasks(
    project_id: web::Path<Uuid>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let tasks = TaskService::get_by_project(project_id.into_inner(), &db).await?;
    Ok(HttpResponse::Ok().json(tasks))
}

#[get("/resource/{resource_id}")]
async fn get_resource_tasks(
    resource_id: web::Path<Uuid>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let tasks = TaskService::get_by_resource(resource_id.into_inner(), &db).await?;
    Ok(HttpResponse::Ok().json(tasks))
}

#[put("/{id}/progress")]
async fn update_task_progress(
    id: web::Path<Uuid>,
    progress: web::Json<i32>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let progress = progress.into_inner();
    if progress < 0 || progress > 100 {
        return Err(ServiceError::ValidationError(
            "Progress must be between 0 and 100".into(),
        ));
    }

    let task = TaskService::update(
        id.into_inner(),
        TaskUpdate {
            progress: Some(BigDecimal::from(progress)),
            ..Default::default()
        },
        &db,
    )
    .await?;

    Ok(HttpResponse::Ok().json(task))
}
