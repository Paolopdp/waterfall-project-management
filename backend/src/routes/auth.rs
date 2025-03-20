use crate::models::auth::{AuthResponse, LoginCredentials};
use crate::models::user::UserCreate;
use crate::services::auth_service::AuthService;
use actix_web::{post, web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = UserCreate,
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User already exists")
    )
)]
#[post("/register")]
pub async fn register(user_create: web::Json<UserCreate>, pool: web::Data<PgPool>) -> HttpResponse {
    match AuthService::register(user_create.into_inner(), &pool).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e.error_response(),
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginCredentials,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Invalid credentials")
    )
)]
#[post("/login")]
pub async fn login(
    credentials: web::Json<LoginCredentials>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match AuthService::login(credentials.into_inner(), &pool).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e.error_response(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(register).service(login));
}
