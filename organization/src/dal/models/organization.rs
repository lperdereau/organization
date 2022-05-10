use super::super::schema::organizations;
use super::super::db;
use super::super::db::LoadPaginated;
use diesel::prelude::*;
use diesel::{Associations, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

#[derive(Serialize, Deserialize, Debug, AsChangeset)]
#[table_name = "organizations"]
pub struct OrganizationMessage {
    pub name: String,
}

#[derive(Identifiable, Queryable, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "organizations"]
pub struct Organization {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    results : Vec<Organization>,
    total_pages : i64
}

impl Organization {
    pub fn find_all(params: Params) -> Result<Response, ApiError> {
        let conn = db::connection()?;
        let query = organizations::table.into_boxed();

        let (users, total_pages) = query
            .load_with_pagination(&conn, params.page, params.page_size)?;

        Ok(Response{results: users, total_pages})
    }

    pub fn create(organization: OrganizationMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let organization = Organization::from(organization);
        diesel::insert_into(organizations::table)
            .values(&organization)
            .execute(&conn)?;

        Ok(organization)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = organizations::table
            .filter(organizations::id.eq(id))
            .first(&conn)?;

        Ok(user)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                organizations::table
                    .filter(organizations::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}

impl From<OrganizationMessage> for Organization {
    fn from(organization: OrganizationMessage) -> Self {
        Organization {
            id: uuid::Uuid::new_v4(),
            name: organization.name,
        }
    }
}
