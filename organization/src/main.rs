#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use clap::Parser;

mod dal;
mod api;
mod config;

use api::{organization, user, group};

#[actix_web::main]
pub async fn main() -> std::io::Result<()>{
    let config = config::Config::parse();
    env_logger::init();
    dal::db::init();

    let server_addr = &format!("{}:{}", config.ip, config.port);
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
