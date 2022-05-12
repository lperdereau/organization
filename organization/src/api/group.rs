use crate::dal::models::{
    group::{Group, GroupMessage, Params},
    user::User,
    user_group::{self, UserGroup},
};
use actix_web::{delete, get, post, web, HttpResponse};
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

#[post("/groups/{group_id}/add/{user_id}/")]
async fn add(message: web::Path<UserGroup>) -> Result<HttpResponse, ApiError> {
    info!("POST /groups/{}/add/{}/", message.group_id, message.user_id);
    let message = message.into_inner();
    let result = UserGroup::create(UserGroup {
        user_id: message.user_id,
        group_id: message.group_id,
    })?;
    Ok(HttpResponse::Created().json(result))
}

#[get("/groups/{group_id}/members/")]
async fn members(
    params: web::Path<user_group::Params>,
) -> Result<HttpResponse, ApiError> {
    info!("GET /groups/{}/members/", params.group_id);
    let result = User::group_member(params.into_inner())?;
    Ok(HttpResponse::Created().json(result))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(get);
    cfg.service(delete);
    cfg.service(add);
    cfg.service(members);
}
