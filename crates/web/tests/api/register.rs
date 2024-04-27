use crate::helpers::spawn_app;
use serde_json::json;

#[tokio::test]
async fn test_register_valid_form() {
    // Setup
    let listening_url = spawn_app("127.0.0.1").await;
    let client = reqwest::Client::new();

    // Action
    let body = json!({
        "email": "test@test.com",
        "username": "test",
        "password": "test"
    });
    let response = client
        .post(&format!("{}/register", listening_url))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    let msg: String = response.text().await.unwrap();
    assert_eq!(msg, "User registered successfully!");
}

#[tokio::test]
async fn test_register_invalid_form() {
    // Setup
    let listening_url = spawn_app("127.0.0.1").await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        (
            json!({
                "username": "test",
                "password": "test"
            }),
            "missing field `email`",
        ),
        (
            json!({
                "email": "test@test.com",
                "password": "test"
            }),
            "missing field `username`",
        ),
        (
            json!({
                "email": "test@test.com",
                "username": "test"
            }),
            "missing field `password`",
        ),
    ];

    for (body, error_message) in test_cases {
        // Action
        let response = client
            .post(&format!("{}/register", listening_url))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .unwrap();

        // Assert
        assert!(response.status().is_client_error());
        let error: String = response.text().await.unwrap();
        assert!(error.contains(error_message));
    }
}
