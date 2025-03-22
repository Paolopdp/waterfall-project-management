use actix_web::web;

pub mod auth;
pub mod lifecycle;
pub mod projects;
pub mod resources;
pub mod tasks;
pub mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::config)
            .configure(projects::config)
            .configure(resources::config)
            .configure(tasks::config)
            .configure(lifecycle::config)
            .configure(users::config),
    );
}
