use log::{debug, error, info};
use std::net::SocketAddr;

use tokio::{
    io::{AsyncWriteExt, Interest},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();
    let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();
    let tcp_listener = TcpListener::bind(addr).await.expect("Failed to bind");
    loop {
        match tcp_listener.accept().await {
            Ok((stream, addr)) => {
                debug!("client address is : {}", addr.to_string());
                handle_connection(stream).await
            }
            Err(err) => {
                log::error!("Failed to read tcp connection for : {:?}", err);
            }
        }
    }
}

async fn handle_connection(stream: TcpStream) {
    let mut stream = stream;
    info!("new client connected.");
    loop {
        match stream.ready(Interest::READABLE).await {
            Ok(ready) => {
                if ready.is_readable() {
                    let mut data = vec![0; 1024];
                    match stream.try_read(&mut data) {
                        Ok(n) => {
                            info!("Read {} bytes", n);
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            // check read size
                            continue;
                        }
                        Err(e) => {
                            error!("Failed to read incomming data for: {:?}", e);
                            // should write back to client.
                            let _ = write_http_response(&mut stream).await;
                            break;
                        }
                    }
                    info!(
                        "got data from client: {}",
                        String::from_utf8_lossy(data.as_slice())
                    );
                    let _ = write_http_response(&mut stream).await;
                    break;
                }
            }
            Err(error) => {
                log::error!("Failed to set interest events for : {:?}", error);
                break;
            }
        }
    }
}

async fn write_http_response(tcpstream: &mut TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nTranster-Encoding: chunked\r\n\r\n{\"name\":\"yaphets\"}";
    let _ = tcpstream.write_all(response).await;
}
