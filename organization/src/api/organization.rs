use crate::dal::models::{
    organization::{NewOrganization, Organization, Params},
    user::User,
    user_organization::UserOrganization,
};
use actix_web::{delete, get, post, HttpResponse, patch, web::ServiceConfig};
use paperclip::actix::web;
use serde_json::json;

use super::ApiError;

#[get("/organizations/")]
async fn list(filters: web::Query<Params>) -> Result<HttpResponse, ApiError> {
    let organizations = Organization::find_all(filters.into_inner())?;
    Ok(HttpResponse::Ok().json(organizations))
}

#[post("/organizations/")]
async fn create(organization: web::Json<NewOrganization>) -> Result<HttpResponse, ApiError> {
    let organization = Organization::create(organization.into_inner())?;
    Ok(HttpResponse::Created().json(organization))
}

#[get("/organizations/{id}/")]
async fn get(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    let organization = Organization::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(organization))
}

#[delete("/organizations/{id}/")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Organization::delete(id.into_inner())?;
    Ok(HttpResponse::NoContent().json(json!({ "deleted": num_deleted })))
}

#[patch("/organizations/{organization_id}/members/")]
async fn add(
    organization_id: web::Path<uuid::Uuid>,
    user_ids: web::Json<Vec<uuid::Uuid>>,
) -> Result<HttpResponse, ApiError> {
    debug!("here");
    let result = UserOrganization::create_multiple(
        user_ids.into_inner().iter().map(|id| {
            UserOrganization {
                user_id: *id,
                organization_id: *organization_id,
            }
        }).collect::<Vec<_>>()
    )?;
    Ok(HttpResponse::Created().json(result))
}

#[get("/organizations/{organization_id}/members/")]
async fn members(
    params: web::Path<crate::dal::models::user_organization::Params>,
) -> Result<HttpResponse, ApiError> {
    let result = User::organization_member(params.into_inner())?;
    Ok(HttpResponse::Ok().json(result))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(add);
    cfg.service(get);
    cfg.service(delete);
    cfg.service(members);
}
