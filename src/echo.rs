//! ping server
//!
//! Reads 'ping' message off the wire on localhost:8007
//! Writes 'pong' massage in reply i.e., this is a limited echo server
//!
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::thread;

use anyhow::Result;

/// Entry point for the echo server.
pub fn run(addr: SocketAddr) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("echo: listening on: {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 1028];
        match stream.read(&mut buf) {
            Ok(bytes) => {
                if bytes == 0 {
                    break; // Connection was closed.
                }

                let read = match std::str::from_utf8(&buf) {
                    Err(_) => {
                        eprintln!("echo: received {} bytes: invalid UTF-8", bytes);
                        continue;
                    }
                    Ok(s) => s,
                };
                if read.contains("ping") {
                    let res = stream.write(b"pong");
                    if res.is_err() {
                        eprintln!("echo: failed to write back to stream")
                    }
                } else {
                    eprintln!("echo: received {} bytes: {}", bytes, read);
                    let _written = stream.write(&buf[0..bytes]).unwrap();
                }
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}
