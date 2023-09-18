#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "info, actix_web=debug, actix_server=debug");
    env_logger::init();

    let listener = std::net::TcpListener::bind("0.0.0.0:8000").expect("Failed to bind port");

    zero2prod::run(listener)?.await
}
