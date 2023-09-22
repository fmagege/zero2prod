use std::net::TcpListener;

use actix_web::{web, App, HttpServer};
use log::info;
use sqlx::PgPool;

use crate::routes::{health_check, subscribe, welcome};

// use crate::{health_check, subscribe, welcome};

const CARGO_PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

pub fn run(
    listener: TcpListener,
    connection_pool: PgPool,
) -> Result<actix_web::dev::Server, std::io::Error> {
    info!(
        "Starting {:?} server at {}",
        CARGO_PKG_NAME.unwrap_or("api"),
        "localhost:8000"
    );

    // wrap connection in a smart pointer
    // we can now clone the connection and pass it to the application factory
    // this is a common pattern in Rust: we want to share a resource across multiple parts of our application, but we don't want to have to worry about the details of memory management.
    // smart pointers are a way to achieve this: they wrap a resource and provide a safe interface to interact with it.
    // in our case, we're using the web::Data smart pointer, which is provided by Actix Web.
    // it allows us to wrap any type and make it available to all handlers.
    // the type we're wrapping is a PgPool, which is a connection pool to our database.
    // we're using a connection pool because it allows us to reuse connections to our database, which is more efficient than creating a new connection every time we need to interact with the database.
    // we're wrapping the connection pool in a web::Data smart pointer because it implements the Clone trait, which means we can clone it and pass it to the application factory.
    // this is important because we want to be able to share the connection pool across multiple threads.

    let connection_pool = web::Data::new(connection_pool);

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(connection_pool.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .service(welcome)
    })
    .listen(listener)?
    .run())
}
