use crate::errors::ServiceError;
use crate::extractors::auth::AuthenticatedUser;
use crate::models::lifecycle::PhaseTransition;
use crate::models::user::UserRole;
use crate::services::lifecycle_service::LifecycleService;
use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/lifecycle")
            .service(transition_phase)
            .service(get_phase_details)
            .service(get_project_lifecycle),
    );
}

/// Transition project to a new phase
#[post("/transition")]
async fn transition_phase(
    auth_user: AuthenticatedUser,
    transition: web::Json<PhaseTransition>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    // Validate request data
    transition.validate()?;

    // Only Project Managers and Admins can transition phases
    match auth_user.role {
        UserRole::Admin | UserRole::ProjectManager => {
            let result = LifecycleService::transition_phase(
                transition.into_inner(),
                auth_user.user_id,
                &pool,
            )
            .await?;
            Ok(HttpResponse::Ok().json(result))
        }
        _ => Err(ServiceError::Forbidden),
    }
}

/// Get details for a specific phase
#[get("/phase/{phase_id}")]
async fn get_phase_details(
    auth_user: AuthenticatedUser,
    phase_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let details = LifecycleService::get_phase_details(*phase_id, &pool).await?;
    Ok(HttpResponse::Ok().json(details))
}

/// Get complete lifecycle history for a project
#[get("/project/{project_id}")]
async fn get_project_lifecycle(
    auth_user: AuthenticatedUser,
    project_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let history = LifecycleService::get_project_lifecycle(*project_id, &pool).await?;
    Ok(HttpResponse::Ok().json(history))
}
