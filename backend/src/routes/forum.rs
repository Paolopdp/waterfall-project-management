use crate::extractors::auth::AuthenticatedUser;
use crate::models::forum::{
    CreateReplyRequest, CreateTagRequest, CreateThreadRequest, ThreadSearchParams,
};
use crate::services::forum_service::ForumService;
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/forum")
            .service(create_thread)
            .service(get_threads)
            .service(create_reply)
            .service(get_thread_replies)
            .service(search_threads)
            .service(create_tag)
            .service(add_thread_tags),
    );
}

#[post("/threads")]
async fn create_thread(
    pool: web::Data<PgPool>,
    auth_user: AuthenticatedUser,
    thread_req: web::Json<CreateThreadRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let forum_service = ForumService::new(pool.get_ref().clone());

    match forum_service
        .create_thread(auth_user.user_id, thread_req.into_inner(), pool.get_ref())
        .await
    {
        Ok(thread) => Ok(HttpResponse::Ok().json(thread)),
        Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
    }
}

#[get("/threads")]
async fn get_threads(
    pool: web::Data<PgPool>,
    _: AuthenticatedUser,
) -> Result<HttpResponse, actix_web::Error> {
    let forum_service = ForumService::new(pool.get_ref().clone());

    match forum_service.get_threads(pool.get_ref()).await {
        Ok(threads) => Ok(HttpResponse::Ok().json(threads)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

#[post("/threads/{thread_id}/replies")]
async fn create_reply(
    pool: web::Data<PgPool>,
    auth_user: AuthenticatedUser,
    thread_id: web::Path<uuid::Uuid>,
    reply_req: web::Json<CreateReplyRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let forum_service = ForumService::new(pool.get_ref().clone());

    match forum_service
        .create_reply(
            auth_user.user_id,
            thread_id.into_inner(),
            reply_req.into_inner(),
            pool.get_ref(),
        )
        .await
    {
        Ok(reply) => Ok(HttpResponse::Ok().json(reply)),
        Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
    }
}

#[get("/threads/{thread_id}/replies")]
async fn get_thread_replies(
    pool: web::Data<PgPool>,
    _: AuthenticatedUser,
    thread_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let forum_service = ForumService::new(pool.get_ref().clone());

    match forum_service
        .get_thread_replies(thread_id.into_inner(), pool.get_ref())
        .await
    {
        Ok(threads) => Ok(HttpResponse::Ok().json(threads)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

#[get("/threads/search")]
async fn search_threads(
    pool: web::Data<PgPool>,
    _: AuthenticatedUser,
    params: web::Query<ThreadSearchParams>,
) -> Result<HttpResponse, actix_web::Error> {
    let forum_service = ForumService::new(pool.get_ref().clone());

    match forum_service
        .search_threads(params.into_inner(), pool.get_ref())
        .await
    {
        Ok(threads) => Ok(HttpResponse::Ok().json(threads)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
}

#[post("/tags")]
async fn create_tag(
    pool: web::Data<PgPool>,
    auth_user: AuthenticatedUser,
    tag_req: web::Json<CreateTagRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let forum_service = ForumService::new(pool.get_ref().clone());

    match forum_service
        .create_tag(
            tag_req.name.clone(),
            tag_req.description.clone(),
            pool.get_ref(),
        )
        .await
    {
        Ok(tag) => Ok(HttpResponse::Ok().json(tag)),
        Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
    }
}

#[post("/threads/{thread_id}/tags")]
async fn add_thread_tags(
    pool: web::Data<PgPool>,
    auth_user: AuthenticatedUser,
    thread_id: web::Path<Uuid>,
    tag_ids: web::Json<Vec<Uuid>>,
) -> Result<HttpResponse, actix_web::Error> {
    let forum_service = ForumService::new(pool.get_ref().clone());

    match forum_service
        .add_thread_tags(thread_id.into_inner(), tag_ids.into_inner(), pool.get_ref())
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Ok(HttpResponse::BadRequest().json(e.to_string())),
    }
}
