use crate::models::project::*;
use crate::routes::projects::*;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Projects
        get_projects,
        get_project,
        create_project,
        update_project,
        delete_project,
        // Resources
        // get_resources,
        // get_resource,
        // create_resource,
        // update_resource,
        // delete_resource,
        // // Tasks
        // get_tasks,
        // get_task,
        // create_task,
        // update_task,
        // delete_task,
    ),
    components(
        schemas(
            Project, ProjectCreate, ProjectUpdate, ProjectStatus,
            // Resource, ResourceCreate, ResourceUpdate,
            // Task, TaskCreate, TaskUpdate, TaskStatus,
        )
    ),
    tags(
        (name = "waterfall-resource-manager", description = "Waterfall Resource Manager API")
    )
)]
pub struct ApiDoc;

pub fn configure_swagger(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi));
}
