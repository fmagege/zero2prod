use std::net::TcpListener;

use actix_web::{web, App, HttpServer};
use log::info;

use crate::routes::{health_check, subscribe, welcome};

// use crate::{health_check, subscribe, welcome};

const CARGO_PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    info!(
        "Starting {:?} server at {}",
        CARGO_PKG_NAME.unwrap_or("api"),
        "localhost:8000"
    );
    Ok(HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .service(welcome)
    })
    .listen(listener)?
    .run())
}
