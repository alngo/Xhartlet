use xhartlet_web::api;

#[tokio::main]
async fn main() {
    let instance = api::run("127.0.0.1").await;
    instance.unwrap().await.unwrap();
}
