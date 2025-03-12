use actix_web::{web, HttpResponse, get, post};
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .service(get_tasks)
            .service(get_task)
            .service(create_task)
    );
}

#[get("")]
async fn get_tasks(_db: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Get all tasks"
    }))
}

#[get("/{id}")]
async fn get_task(path: web::Path<String>, _db: web::Data<PgPool>) -> HttpResponse {
    let id = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Get task {}", id)
    }))
}

#[post("")]
async fn create_task(_db: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Created().json(serde_json::json!({
        "message": "Create new task"
    }))
}
