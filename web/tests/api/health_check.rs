use crate::helpers::spawn_app;

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
