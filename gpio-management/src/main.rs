// src/main.rs
#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod services;
mod libs;

const BASE_URL_PATH: &str = "/v1.0/gpio/management";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
        .service(
            web::scope(BASE_URL_PATH).configure(services::init_routes)
        )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("ACTIX_HOST").expect("host not set");
            let port = env::var("ACTIX_PORT").expect("port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await
}
