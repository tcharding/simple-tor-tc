use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use anyhow::Result;
use tokio::net::TcpStream;

const TC_PORT: u16 = 9051;

pub async fn connect() -> Result<TcpStream> {
    let sock = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), TC_PORT);
    let stream = TcpStream::connect(sock).await?;
    Ok(stream)
}
