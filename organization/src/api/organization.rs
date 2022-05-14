use crate::dal::models::{
    organization::{Organization, NewOrganization, Params},
    user_organization::UserOrganization,
    user::User,
};
use actix_web::{delete, get, post, web, HttpResponse};
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

#[post("/organizations/{organization_id}/add/{user_id}/")]
async fn add(
    message: web::Path<UserOrganization>,
) -> Result<HttpResponse, ApiError> {
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
