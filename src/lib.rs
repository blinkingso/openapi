use hyper::{
    server::Server,
    service::{make_service_fn, service_fn},
    Body, Error, Request, Response, StatusCode,
};
use log::info;
use std::net::SocketAddr;
pub async fn run() -> std::io::Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, Error>(service_fn(|req| async move {
            Ok::<_, Error>(dispatch_handler(req).await)
        }))
    }));
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    let _ = graceful.await.expect("failed to start server");
    Ok(())
}

async fn dispatch_handler(req: Request<Body>) -> Response<Body> {
    {
        let uri = req.uri().to_string();
        info!("receive http request from client for: {}", uri);
        // log headers
        let headers = req.headers();
        for header in headers.iter() {
            let hv = String::from_utf8_lossy(header.1.as_bytes()).to_string();
            info!("header=>{}:{}", header.0, hv);
        }
    }
    // uri matches.
    let uri = req.uri().to_string();
    match uri.as_str() {
        "/hello" => hello().await,
        "/world" => world().await,
        "/health" => health_check().await,
        unknown => not_found(unknown).await,
    }
}

async fn hello() -> Response<Body> {
    Response::new("hello".into())
}
async fn world() -> Response<Body> {
    Response::new("world".into())
}

async fn not_found(not_found: &str) -> Response<Body> {
    let msg = format!("path {} not found", not_found);
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(msg.into())
        .unwrap()
}

pub async fn health_check() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from("OK"))
        .unwrap()
}

async fn shutdown_signal() {
    info!("receiving shutdown signal, system is starting to exit.");
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    info!("system shutdown successfully...");
}
