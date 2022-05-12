use super::super::db;
use super::super::schema::users_groups;
use diesel::prelude::*;
use diesel::{Associations, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

#[derive(Queryable, Insertable, Debug, Associations, Deserialize, Serialize)]
#[belongs_to(super::user::User)]
#[belongs_to(super::group::Group)]
#[table_name = "users_groups"]
pub struct UserGroup {
    pub user_id: uuid::Uuid,
    pub group_id: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub group_id: uuid::Uuid,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    results: Vec<UserGroup>,
    total_pages: i64,
}

impl UserGroup {
    pub fn create(user_group: UserGroup) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user_group = UserGroup::from(user_group);
        diesel::insert_into(users_groups::table)
            .values(&user_group)
            .execute(&conn)?;

        Ok(user_group)
    }

    pub fn delete(user_id: uuid::Uuid, groups_id: uuid::Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            users_groups::table.filter(
                users_groups::user_id
                    .eq(user_id)
                    .and(users_groups::group_id.eq(groups_id)),
            ),
        )
        .execute(&conn)?;

        Ok(res)
    }
}
