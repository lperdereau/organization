use super::super::db;
use super::super::schema::users_organizations;
use diesel::prelude::*;
use diesel::{Associations, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

#[derive(Queryable, Insertable, Debug, Associations, Deserialize, Serialize)]
#[belongs_to(super::user::User)]
#[belongs_to(super::organization::Organization)]
#[table_name = "users_organizations"]
pub struct UserOrganization {
    pub user_id: uuid::Uuid,
    pub organization_id: uuid::Uuid,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub organization_id: uuid::Uuid,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    results: Vec<UserOrganization>,
    total_pages: i64,
}

impl UserOrganization {
    pub fn create(user_organization: UserOrganization) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user_organization = UserOrganization::from(user_organization);
        diesel::insert_into(users_organizations::table)
            .values(&user_organization)
            .execute(&conn)?;

        Ok(user_organization)
    }

    pub fn delete(user_id: uuid::Uuid, organization_id: uuid::Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
            users_organizations::table.filter(
                users_organizations::user_id
                    .eq(user_id)
                    .and(users_organizations::organization_id.eq(organization_id)),
            ),
        )
        .execute(&conn)?;

        Ok(res)
    }
}
