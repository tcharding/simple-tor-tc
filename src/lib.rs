use std::net::SocketAddr;
use anyhow::Result;
use tokio::net::{TcpStream};
use tokio_socks::tcp::Socks5Stream;
use tokio_socks::IntoTargetAddr;

pub async fn connect_tor_cp(addr: SocketAddr) -> Result<TcpStream> {
    let sock = TcpStream::connect(addr).await?;
    Ok(sock)
}

pub async fn connect_tor_socks_proxy<'a>(proxy: SocketAddr, dest: impl IntoTargetAddr<'a>) -> Result<TcpStream> {
    let sock = Socks5Stream::connect(proxy, dest).await?;
    Ok(sock.into_inner())
}
