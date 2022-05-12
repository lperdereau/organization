use crate::dal::models::{
    organization::{Organization, OrganizationMessage, Params},
    user_organization::UserOrganization,
    user::User,
};
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;

use super::ApiError;

#[get("/organizations/")]
async fn list(filters: web::Query<Params>) -> Result<HttpResponse, ApiError> {
    info!("GET /organizations/");
    let organizations = Organization::find_all(filters.into_inner())?;
    Ok(HttpResponse::Ok().json(organizations))
}

#[post("/organizations/")]
async fn create(organization: web::Json<OrganizationMessage>) -> Result<HttpResponse, ApiError> {
    info!("POST /organizations/");
    let organization = Organization::create(organization.into_inner())?;
    Ok(HttpResponse::Created().json(organization))
}


#[get("/organizations/{id}/")]
async fn get(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    info!("GET /organizations/{id}/");
    let organization = Organization::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(organization))
}

#[delete("/organizations/{id}/")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    info!("DELETE /organizations/{id}/");
    let num_deleted = Organization::delete(id.into_inner())?;
    Ok(HttpResponse::NoContent().json(json!({ "deleted": num_deleted })))
}

#[post("/organizations/{organization_id}/add/{user_id}/")]
async fn add(
    message: web::Path<UserOrganization>,
) -> Result<HttpResponse, ApiError> {
    info!("POST /organizations/{}/add/{}/", message.organization_id, message.user_id);
    let message = message.into_inner();
    let result = UserOrganization::create(UserOrganization {
        user_id: message.user_id,
        organization_id: message.organization_id,
    })?;
    Ok(HttpResponse::Created().json(result))
}

#[get("/organizations/{organization_id}/members/")]
async fn members(
    params: web::Path<crate::dal::models::user_organization::Params>,
) -> Result<HttpResponse, ApiError> {
    info!("GET /organizations/{}/members/", params.organization_id);
    let result = User::organization_member(params.into_inner())?;
    Ok(HttpResponse::Created().json(result))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(add);
    cfg.service(get);
    cfg.service(delete);
    cfg.service(members);
}
