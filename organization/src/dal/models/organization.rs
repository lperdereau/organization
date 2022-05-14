use super::super::db::{self, Paginate};
use super::super::schema::organizations;
use super::Response;
use diesel::prelude::*;
use diesel::{Associations, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewOrganization {
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

impl Organization {
    pub fn find_all(params: Params) -> Result<Response<Organization>, ApiError> {
        let conn = db::connection()?;
        let mut query = organizations::table
            .into_boxed()
            .paginate(params.page.unwrap_or_else(|| 1));

        let page_size = params.page_size;
        if let Some(page_size) = page_size {
            use std::cmp::min;
            query = query.per_page(min(page_size, 25));
        }

        let (organizations, total_pages) = query.load_and_count_pages::<Organization>(&conn)?;

        Ok(Response {
            results: organizations,
            total_pages,
        })
    }

    pub fn create(organization: NewOrganization) -> Result<Self, ApiError> {
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

        let res =
            diesel::delete(organizations::table.filter(organizations::id.eq(id))).execute(&conn)?;

        Ok(res)
    }
}

impl From<NewOrganization> for Organization {
    fn from(organization: NewOrganization) -> Self {
        Organization {
            id: uuid::Uuid::new_v4(),
            name: organization.name,
        }
    }
}
