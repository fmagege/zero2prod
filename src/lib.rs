use std::net::TcpListener;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Let's start simple: we always return a 200 OK
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    info!("Starting server at {}", "localhost:8000");

    Ok(HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .service(welcome)
    })
    .listen(listener)?
    .run())
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn welcome(info: web::Path<Info>) -> impl Responder {
    // let (user_id, friend) = info.into_inner();
    format!("Welcome {}, user_id {}!", info.friend, info.user_id)
}
