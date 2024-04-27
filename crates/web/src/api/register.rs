use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterForm {
    email: String,
    username: String,
    password: String,
}

pub async fn register(Json(payload): Json<RegisterForm>) -> impl IntoResponse {
    println!(
        "email: {}, username: {}, password: {}",
        payload.email, payload.username, payload.password
    );
    (StatusCode::OK, "User registered successfully!")
}
