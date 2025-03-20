use crate::errors::ServiceError;
use crate::models::user::{User, UserCreate, UserUpdate};
use crate::services::user_service::UserService;
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = UserCreate,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/api/users")]
pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<UserCreate>,
) -> Result<HttpResponse, actix_web::Error> {
    user.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;
    let user = UserService::create(user.into_inner(), &pool).await?;
    Ok(HttpResponse::Created().json(user))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
#[get("/api/users/{id}")]
pub async fn get_user(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = UserService::get_by_id(id.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    put,
    path = "/api/users/{id}",
    request_body = UserUpdate,
    responses(
        (status = 200, description = "User updated successfully", body = User),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
#[put("/api/users/{id}")]
pub async fn update_user(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    user: web::Json<UserUpdate>,
) -> Result<HttpResponse, actix_web::Error> {
    user.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;
    let user = UserService::update(id.into_inner(), user.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    )
)]
#[delete("/api/users/{id}")]
pub async fn delete_user(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    UserService::delete(id.into_inner(), &pool).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/api/users")]
pub async fn list_users(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let users = UserService::list(&pool).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user)
        .service(get_user)
        .service(update_user)
        .service(delete_user)
        .service(list_users);
}
