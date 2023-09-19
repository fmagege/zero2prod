use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
