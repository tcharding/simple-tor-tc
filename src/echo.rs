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
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                eprintln!(
                    "echo: received {} bytes: {}",
                    n,
                    std::str::from_utf8(&read).unwrap()
                );
                let _written = stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}
