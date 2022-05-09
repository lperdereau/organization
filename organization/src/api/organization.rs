use actix_web::{web, get, post, delete, HttpResponse};
use crate::dal::models::organization::{Organization, Params, OrganizationMessage};
use serde_json::json;

use super::ApiError;

#[get("/organizations/")]
async fn list(filters: web::Query<Params>) -> Result<HttpResponse, ApiError> {
    info!("GET /users/");
    let organizations = Organization::find_all(filters.into_inner())?;
    Ok(HttpResponse::Ok().json(organizations))
}

#[post("/organizations/")]
async fn create(organization: web::Json<OrganizationMessage>) -> Result<HttpResponse, ApiError> {
    info!("POST /users/");
    let organization = Organization::create(organization.into_inner())?;
    Ok(HttpResponse::Created().json(organization))
}

#[get("/organizations/{id}/")]
async fn get(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    info!("GET /users/{id}/");
    let organization = Organization::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(organization))
}

#[delete("/organizations/{id}/")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    info!("DELETE /users/{id}/");
    let num_deleted = Organization::delete(id.into_inner())?;
    Ok(HttpResponse::NoContent().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(get);
    cfg.service(delete);
}
