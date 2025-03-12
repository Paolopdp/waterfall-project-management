use actix_web::web;

pub mod projects;
pub mod resources;
pub mod tasks;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(projects::config)
            .configure(resources::config)
            .configure(tasks::config),
    );
}
