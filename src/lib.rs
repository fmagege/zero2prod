use std::net::TcpListener;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use log::info;

async fn health_check(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    info!("Starting server at {}", "localhost:8000");

    Ok(
        HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
            .listen(listener)?
            .run(),
    )
}
