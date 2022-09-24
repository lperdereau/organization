use super::super::db::{self, Paginate};
use super::super::schema::groups;
use super::Response;
use diesel::prelude::*;
use diesel::{Associations, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewGroup {
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

impl Group {
    pub fn find_all(params: Params) -> Result<Response<Group>, ApiError> {
        let conn = db::connection()?;
        let mut query = groups::table
            .into_boxed()
            .paginate(params.page.unwrap_or_else(|| 1));

        let page_size = params.page_size;
        if let Some(page_size) = page_size {
            use std::cmp::min;
            query = query.per_page(min(page_size, 25));
        }

        let (groups, total_pages) = query.load_and_count_pages::<Group>(&conn)?;

        Ok(Response {
            results: groups,
            total_pages,
        })
    }

    pub fn create(group: NewGroup) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let group = Group::from(group);
        diesel::insert_into(groups::table)
            .values(&group)
            .execute(&conn)?;

        Ok(group)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = groups::table.filter(groups::id.eq(id)).first(&conn)?;

        Ok(user)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(groups::table.filter(groups::id.eq(id))).execute(&conn)?;

        Ok(res)
    }
}

impl From<NewGroup> for Group {
    fn from(group: NewGroup) -> Self {
        Group {
            id: uuid::Uuid::new_v4(),
            name: group.name,
            organization_id: group.organization_id,
        }
    }
}
