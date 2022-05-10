use super::super::schema::users;
use super::super::db;
use super::super::db::LoadPaginated;
use diesel::prelude::*;
use diesel::{Associations, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

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
    results : Vec<User>,
    total_pages : i64
}

impl User {
    pub fn find_all(params: Params) -> Result<Response, ApiError> {
        let conn = db::connection()?;
        let query = users::table.into_boxed();

        let (users, total_pages) = query
            .load_with_pagination(&conn, params.page, params.page_size)?;

        Ok(Response{results: users, total_pages})
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

        let user = users::table
            .filter(users::id.eq(id))
            .first(&conn)?;

        Ok(user)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                users::table
                    .filter(users::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
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
