use serde::Deserialize;
use axum::{Json, response::IntoResponse, http::StatusCode};

#[derive(Deserialize)]
pub struct RegisterForm {
    email: String,
    username: String,
    password: String,
}

pub async fn register(Json(payload): Json<RegisterForm>) -> impl IntoResponse {
    println!("email: {}, username: {}, password: {}", payload.email, payload.username, payload.password);
    

    (StatusCode::OK, "")
}
