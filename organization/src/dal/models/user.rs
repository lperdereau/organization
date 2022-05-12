use super::super::db::{self, Paginate};
use super::super::schema::users;
use super::user_group::UserGroup;
use super::user_organization::UserOrganization;
use diesel::prelude::*;
use diesel::{Associations, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;
use crate::dal::schema::{users_organizations, users_groups};

#[derive(Serialize, Deserialize, Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserMessage {
    pub name: String,
}

#[derive(Identifiable, Queryable, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "users"]
pub struct User {
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
    results: Vec<User>,
    total_pages: i64,
}

impl User {
    pub fn find_all(params: Params) -> Result<Response, ApiError> {
        let conn = db::connection()?;
        let mut query = users::table
            .into_boxed()
            .paginate(params.page.unwrap_or_else(|| 1));

        let page_size = params.page_size;
        if let Some(page_size) = page_size {
            use std::cmp::min;
            query = query.per_page(min(page_size, 25));
        }

        let (users, total_pages) = query.load_and_count_pages::<User>(&conn)?;

        Ok(Response {
            results: users,
            total_pages,
        })
    }

    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = User::from(user);
        diesel::insert_into(users::table)
            .values(&user)
            .execute(&conn)?;

        Ok(user)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = users::table.filter(users::id.eq(id)).first(&conn)?;

        Ok(user)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&conn)?;

        Ok(res)
    }

    pub fn organization_member(
        params: super::user_organization::Params,
    ) -> Result<Response, ApiError> {
        let conn = db::connection()?;

        let mut query = users_organizations::table
            .inner_join(users::table.on(users::id.eq(users_organizations::user_id)))
            .filter(users_organizations::organization_id.eq(params.organization_id))
            .paginate(params.page.unwrap_or_else(|| 1));

        let page_size = params.page_size;
        if let Some(page_size) = page_size {
            use std::cmp::min;
            query = query.per_page(min(page_size, 25));
        }

        let (users_vec, total_pages) =
            query.load_and_count_pages::<(UserOrganization, User)>(&conn)?;
        Ok(Response {
            results: users_vec
                .into_iter()
                .map(|x| x.1)
                .rev()
                .collect::<Vec<User>>(),
            total_pages,
        })
    }

    pub fn group_member(
        params: super::user_group::Params,
    ) -> Result<Response, ApiError> {
        let conn = db::connection()?;

        let mut query = users_groups::table
            .inner_join(users::table.on(users::id.eq(users_groups::user_id)))
            .filter(users_groups::group_id.eq(params.group_id))
            .paginate(params.page.unwrap_or_else(|| 1));

        let page_size = params.page_size;
        if let Some(page_size) = page_size {
            use std::cmp::min;
            query = query.per_page(min(page_size, 25));
        }

        let (users_vec, total_pages) =
            query.load_and_count_pages::<(UserGroup, User)>(&conn)?;
        Ok(Response {
            results: users_vec
                .into_iter()
                .map(|x| x.1)
                .rev()
                .collect::<Vec<User>>(),
            total_pages,
        })
    }
}

impl From<UserMessage> for User {
    fn from(user: UserMessage) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            name: user.name,
        }
    }
}
