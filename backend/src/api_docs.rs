use crate::models::{project::*, user::*};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::projects::create_project,
        crate::routes::projects::get_project,
        crate::routes::projects::update_project,
        crate::routes::projects::delete_project,
        crate::routes::projects::get_projects,
        crate::routes::users::create_user,
        crate::routes::users::get_user,
        crate::routes::users::update_user,
        crate::routes::users::delete_user,
        crate::routes::users::list_users,
    ),
    components(
        schemas(
            Project,
            ProjectCreate,
            ProjectUpdate,
            ProjectStatus,
            User,
            UserCreate,
            UserUpdate,
            UserRole,
        )
    ),
    tags(
        (name = "projects", description = "Project management endpoints"),
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;

pub fn configure_swagger(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
}
