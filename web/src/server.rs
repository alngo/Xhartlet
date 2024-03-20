use std::{convert::Infallible, net::SocketAddr};

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::{server::conn::http1, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health_check") => Ok(Response::new(Full::new(Bytes::from("OK")))),
        _ => Ok(not_found),
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        if let Err(err) = http1::Builder::new()
            .serve_connection(io, service_fn(handle_request))
            .await
        {
            eprintln!("Error serving connection: {:?}", err);
        }
    }
}
