mod health_check;

use health_check::health_check;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router, serve::Serve};

pub async fn run() -> Result<Serve<Router, Router>, std::io::Error> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let server = axum::serve(listener, app());
    Ok(server)
}

pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}
