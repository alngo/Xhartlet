use super::*;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "")
}
