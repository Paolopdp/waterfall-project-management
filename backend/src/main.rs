use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod api_docs;
mod db;
mod errors;
mod extractors;
mod models;
mod routes;
mod services;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("./db/migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run migrations");

    log::info!("Starting server at http://127.0.0.1:3001");
    log::info!("Swagger UI available at http://127.0.0.1:3001/swagger-ui/");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::config)
            .configure(api_docs::configure_swagger)
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
}
