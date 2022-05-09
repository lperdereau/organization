use actix_web::{web, get, post, delete, HttpResponse};
use crate::dal::models::user::{User, Params, UserMessage};
use serde_json::json;

use super::ApiError;

#[get("/users/")]
async fn list(filters: web::Query<Params>) -> Result<HttpResponse, ApiError> {
    info!("GET /users/");
    let users = User::find_all(filters.into_inner())?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/users/")]
async fn create(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    info!("POST /users/");
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Created().json(user))
}

#[get("/users/{id}/")]
async fn get(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    info!("GET /users/{id}/");
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}/")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    info!("DELETE /users/{id}/");
    let num_deleted = User::delete(id.into_inner())?;
    Ok(HttpResponse::NoContent().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(get);
    cfg.service(delete);
}
