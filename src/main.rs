use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "info, actix_web=debug, actix_server=debug");
    env_logger::init();

    let configuration = get_configuration().expect("Failed to read configuration.");
    // let connection = PgConnection::connect(&configuration.database.connection_string())
    //     .await
    //     .expect("Failed to connect to Postgres.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener, connection_pool)?.await
}

// let connection_pool = zero2prod::db::get_connection_pool(&configuration.database)
//     .await
//     .expect("Failed to connect to Postgres.");
// let connection_pool = zero2prod::db::get_connection_pool(&configuration.database)
