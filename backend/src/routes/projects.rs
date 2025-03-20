use crate::errors::ServiceError;
use crate::models::project::{Project, ProjectCreate, ProjectUpdate};
use crate::models::user::UserRole;
use crate::services::project_service::ProjectService;
use crate::extractors::auth::AuthenticatedUser;
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .service(get_projects)
            .service(get_project)
            .service(create_project)
            .service(update_project)
            .service(delete_project),
    );
}

/// Get all projects
#[utoipa::path(
    get,
    path = "/api/projects",
    responses(
        (status = 200, description = "List of projects", body = Vec<Project>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("")]
async fn get_projects(
    auth_user: AuthenticatedUser,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, ServiceError> {
    // Only allow certain roles to access this endpoint
    match auth_user.role {
        UserRole::Admin | UserRole::ProjectManager | UserRole::Developer => {
            let projects = ProjectService::get_all(&pool).await?;
            Ok(HttpResponse::Ok().json(projects))
        },
        _ => Err(ServiceError::Forbidden)
    }
}

/// Get project by ID
#[utoipa::path(
    get,
    path = "/api/projects/{id}",
    params(
        ("id" = String, Path, description = "Project UUID")
    ),
    responses(
        (status = 200, description = "Project found", body = Project),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/{id}")]
async fn get_project(
    path: web::Path<String>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let id = path.into_inner();
    let project_id = Uuid::parse_str(&id)
        .map_err(|_| ServiceError::BadRequest("Invalid UUID format".to_string()))?;

    let project = ProjectService::get_by_id(project_id, &db).await?;
    Ok(HttpResponse::Ok().json(project))
}

/// Create a new project
#[utoipa::path(
    post,
    path = "/api/projects",
    request_body = ProjectCreate,
    responses(
        (status = 201, description = "Project created successfully", body = Project),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("")]
async fn create_project(
    auth_user: AuthenticatedUser,
    project: web::Json<ProjectCreate>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, ServiceError> {
    match auth_user.role {
        UserRole::Admin | UserRole::ProjectManager => {
            let project = ProjectService::create(project.into_inner(), &pool).await?;
            Ok(HttpResponse::Created().json(project))
        },
        _ => Err(ServiceError::Forbidden)
    }
}

/// Update an existing project
#[utoipa::path(
    put,
    path = "/api/projects/{id}",
    params(
        ("id" = String, Path, description = "Project UUID")
    ),
    request_body = ProjectUpdate,
    responses(
        (status = 200, description = "Project updated successfully", body = Project),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/{id}")]
async fn update_project(
    path: web::Path<String>,
    project: web::Json<ProjectUpdate>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let id = path.into_inner();
    let project_id = Uuid::parse_str(&id)
        .map_err(|_| ServiceError::BadRequest("Invalid UUID format".to_string()))?;

    // Validate the request body
    project
        .validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;

    let updated_project = ProjectService::update(project_id, project.into_inner(), &db).await?;
    Ok(HttpResponse::Ok().json(updated_project))
}

/// Delete a project
#[utoipa::path(
    delete,
    path = "/api/projects/{id}",
    params(
        ("id" = String, Path, description = "Project UUID")
    ),
    responses(
        (status = 204, description = "Project deleted successfully"),
        (status = 404, description = "Project not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/{id}")]
async fn delete_project(
    path: web::Path<String>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let id = path.into_inner();
    let project_id = Uuid::parse_str(&id)
        .map_err(|_| ServiceError::BadRequest("Invalid UUID format".to_string()))?;

    ProjectService::delete(project_id, &db).await?;
    Ok(HttpResponse::NoContent().finish())
}
