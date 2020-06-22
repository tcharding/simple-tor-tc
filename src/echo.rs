//! ping server
//!
//! Reads 'ping' message off the wire on localhost:8007
//! Writes 'pong' massage in reply i.e., this is a limited echo server
//!
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use anyhow::Result;

/// Entry point for the echo server.
pub async fn run(addr: SocketAddr) -> Result<()> {
    let mut listener = TcpListener::bind(addr).await?;
    println!("echo: listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        handle_client(socket).await?;
    }
}

async fn handle_client(mut stream: TcpStream) -> Result<()> {
    loop {
        let mut buf = [0; 1028];
        match stream.read(&mut buf).await {
            Ok(bytes) => {
                if bytes == 0 {
                    return Ok(()); // Connection was closed.
                }

                let read = match std::str::from_utf8(&buf) {
                    Err(_) => {
                        eprintln!("echo: received {} bytes: invalid UTF-8", bytes);
                        continue;
                    },
                    Ok(s) => s,
                };

                let res = if read.contains("ping") {
                    stream.write(b"pong").await
                } else {
                    stream.write(&buf[0..bytes]).await
                };

                if res.is_err() {
                    eprintln!("echo: failed to write back to stream")
                }
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}
