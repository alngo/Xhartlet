use xhartlet_web::api::app;

pub async fn spawn_app(host: impl Into<String>) -> String {
    let host = host.into();
    let listener = tokio::net::TcpListener::bind(format!("{}:0", host))
        .await
        .unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async {
        axum::serve(listener, app()).await.unwrap();
    });
    format!("http://{}:{}", host, port)
}
