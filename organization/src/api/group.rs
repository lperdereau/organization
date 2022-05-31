use crate::dal::models::{
    group::{Group, NewGroup, Params},
    user::User,
    user_group::{self, UserGroup},
};
use actix_web::{delete, get, post, patch, web, HttpResponse};
use serde_json::json;

use super::ApiError;

#[get("/groups/")]
async fn list(filters: web::Query<Params>) -> Result<HttpResponse, ApiError> {
    let groups = Group::find_all(filters.into_inner())?;
    Ok(HttpResponse::Ok().json(groups))
}

#[post("/groups/")]
async fn create(group: web::Json<NewGroup>) -> Result<HttpResponse, ApiError> {
    let group = Group::create(group.into_inner())?;
    Ok(HttpResponse::Created().json(group))
}

#[get("/groups/{id}/")]
async fn get(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    let group = Group::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(group))
}

#[delete("/groups/{id}/")]
async fn delete(id: web::Path<uuid::Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Group::delete(id.into_inner())?;
    Ok(HttpResponse::NoContent().json(json!({ "deleted": num_deleted })))
}

#[patch("/groups/{group_id}/members/")]
async fn add(
    group_id: web::Path<uuid::Uuid>,
    user_ids: web::Json<Vec<uuid::Uuid>>,
) -> Result<HttpResponse, ApiError> {
    debug!("here");
    let result = UserGroup::create_multiple(
        user_ids
            .into_inner()
            .iter()
            .map(|id| UserGroup {
                user_id: *id,
                group_id: *group_id,
            })
            .collect::<Vec<_>>(),
    )?;
    Ok(HttpResponse::Created().json(result))
}

#[get("/groups/{group_id}/members/")]
async fn members(
    params: web::Path<user_group::Params>,
) -> Result<HttpResponse, ApiError> {
    let result = User::group_member(params.into_inner())?;
    Ok(HttpResponse::Ok().json(result))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(get);
    cfg.service(delete);
    cfg.service(add);
    cfg.service(members);
}
