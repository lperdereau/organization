#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
mod dal;
mod api;
mod config;

use api::{organization, user, group};
use config::CONFIG;

#[actix_web::main]
pub async fn main() -> std::io::Result<()>{
    env_logger::init();
    dal::db::init();

    let server_addr = &format!("{}:{}", CONFIG.ip, CONFIG.port);
    let server = HttpServer::new(|| {
        App::new()
            .configure(organization::init_routes)
            .configure(user::init_routes)
            .configure(group::init_routes)
    })
    .bind(server_addr)?;

    info!("Starting server");
    info!("Listening: http://{}/", server_addr);
    server
        .run()
        .await
}
