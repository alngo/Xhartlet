mod health_check;

use health_check::health_check;

use axum::{http::StatusCode, response::IntoResponse, routing::get, serve::Serve, Router};
use tokio::net::TcpListener;

pub async fn run(host: impl Into<String>) -> Result<Serve<Router, Router>, std::io::Error> {
    let host = host.into();
    let listener = TcpListener::bind(format!("{}:0", host)).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let server = axum::serve(listener, app());
    Ok(server)
}

pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}
