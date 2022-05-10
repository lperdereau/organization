use super::super::schema::groups;
use super::super::db;
use super::super::db::LoadPaginated;
use diesel::prelude::*;
use diesel::{Associations, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

#[derive(Serialize, Deserialize, Debug, AsChangeset)]
#[table_name = "groups"]
pub struct GroupMessage {
    pub organization_id: uuid::Uuid,
    pub name: String,
}

#[derive(Identifiable, Queryable, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "groups"]
#[belongs_to(super::organization::Organization)]
pub struct Group {
    pub id: uuid::Uuid,
    pub organization_id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    results : Vec<Group>,
    total_pages : i64
}

impl Group {
    pub fn find_all(params: Params) -> Result<Response, ApiError> {
        let conn = db::connection()?;
        let query = groups::table.into_boxed();

        let (groups, total_pages) = query
            .load_with_pagination(&conn, params.page, params.page_size)?;

        Ok(Response{results: groups, total_pages})
    }

    pub fn create(group: GroupMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let group = Group::from(group);
        diesel::insert_into(groups::table)
            .values(&group)
            .execute(&conn)?;

        Ok(group)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = groups::table
            .filter(groups::id.eq(id))
            .first(&conn)?;

        Ok(user)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                groups::table
                    .filter(groups::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}

impl From<GroupMessage> for Group {
    fn from(group: GroupMessage) -> Self {
        Group {
            id: uuid::Uuid::new_v4(),
            name: group.name,
            organization_id: group.organization_id,
        }
    }
}
