use actix_web::{web, get, post, delete, HttpResponse};
use crate::dal::models::group::{Group, Params, GroupMessage};
use serde_json::json;

use super::ApiError;

#[get("/groups/")]
async fn list(filters: web::Query<Params>) -> Result<HttpResponse, ApiError> {
    info!("GET /groups/");
    let groups = Group::find_all(filters.into_inner())?;
    Ok(HttpResponse::Ok().json(groups))
}

#[post("/groups/")]
async fn create(group: web::Json<GroupMessage>) -> Result<HttpResponse, ApiError> {
    info!("POST /groups/");
    let group = Group::create(group.into_inner())?;
    Ok(HttpResponse::Created().json(group))
}

#[get("/groups/{id}/")]
async fn get(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    info!("GET /groups/{id}/");
    let group = Group::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(group))
}

#[delete("/groups/{id}/")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    info!("DELETE /groups/{id}/");
    let num_deleted = Group::delete(id.into_inner())?;
    Ok(HttpResponse::NoContent().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(get);
    cfg.service(delete);
}
