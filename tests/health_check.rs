use zero2prod::run;

fn spawn_app() -> String {
    // let db_pool = configure_database().await?;
    // let server = run(db_pool.clone()).await?;
    // let _ = tokio::spawn(server);
    // Ok(db_pool)

    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan for an available port which will then be bound to the application.
    // This is useful for tests, as it allows us to run multiple tests in parallel without having to worry about port conflicts.
    // The OS will assign a random port to each test, and we can then use that port to send requests to the application.
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
