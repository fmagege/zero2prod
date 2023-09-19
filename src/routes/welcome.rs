use actix_web::{get, web, Responder};
use serde::Deserialize;

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
