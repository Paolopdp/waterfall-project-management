use actix_web::{web, HttpResponse, get, post};
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources")
            .service(get_resources)
            .service(get_resource)
            .service(create_resource)
    );
}

#[get("")]
async fn get_resources(_db: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Get all resources"
    }))
}

#[get("/{id}")]
async fn get_resource(path: web::Path<String>, _db: web::Data<PgPool>) -> HttpResponse {
    let id = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Get resource {}", id)
    }))
}

#[post("")]
async fn create_resource(_db: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Created().json(serde_json::json!({
        "message": "Create new resource"
    }))
}
