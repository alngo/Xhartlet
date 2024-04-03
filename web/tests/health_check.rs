use xhartlet_web::api::app;

#[tokio::test]
async fn test_health_check() {
    // Setup
    let listening_url = spawn_app("127.0.0.1").await;
    let client = reqwest::Client::new();

    // Action
    let response = client
        .get(format!("{}/health_check", listening_url))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    
}

async fn spawn_app(host: impl Into<String>) -> String {
    let host = host.into();
    let listener = tokio::net::TcpListener::bind(format!("{}:0", host)).await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async {
        axum::serve(listener, app()).await.unwrap();
    });
    format!("http://{}:{}", host, port)
}

