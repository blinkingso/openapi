use std::{convert::Infallible, future::Future, net::SocketAddr};

use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use log::debug;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();
    let tcp_listener = TcpListener::bind(&addr).await?;
    loop {
        let (tcp_stream, client_addr) = tcp_listener.accept().await?;
        tokio::task::spawn(async move {
            Http::new()
                .http1_only(true)
                .http1_keep_alive(true)
                .serve_connection(
                    tcp_stream,
                    service_fn(move |req| dispatch_handler(req, client_addr)),
                )
                .await
        });
    }
}

async fn dispatch_handler(
    req: Request<Body>,
    client_addr: SocketAddr,
) -> Result<Response<Body>, Infallible> {
    debug!(
        "Receiving remote request to {}, remote ip address is {}",
        req.uri(),
        client_addr.ip().to_string()
    );

    Ok(Response::new(Body::from("Ok")))
}
