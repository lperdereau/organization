#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
mod api;
mod config;
mod dal;

use api::{group, organization, user};
use config::CONFIG;

fn generate_cors() -> Cors {
    Cors::default()
        .max_age(3600)
        .allow_any_method()
        .allowed_origin_fn(|origin, _req_head| -> bool {
            let allowed_origins: Vec<String> = CONFIG
                .allowed_origins
                .split(',')
                .map(|s| s.to_string())
                .collect();
            if allowed_origins.iter().any(|e| e == "*") {
                return true;
            }
            return allowed_origins.iter().any(|e| origin == e);
        })
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env_logger::init();
    dal::db::init();

    let server_addr = &format!("{}:{}", CONFIG.ip, CONFIG.port);
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(generate_cors())
            .configure(organization::init_routes)
            .configure(user::init_routes)
            .configure(group::init_routes)
    })
    .bind(server_addr)?;

    info!("Starting server");
    info!("Listening: http://{}/", server_addr);
    server.run().await
}
