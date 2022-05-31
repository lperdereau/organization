use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;

pub mod organization;
pub mod user;
pub mod user_organization;
pub mod user_group;
pub mod group;

#[derive(Debug, Deserialize, Serialize, Apiv2Schema)]
pub struct Response<T> {
    results: Vec<T>,
    total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    limit: u8,
    offset: i64
}

impl Default for Page {
    fn default() -> Self {
        Self { limit: CONFIG.page_limit, offset: 0 }
    }
}
