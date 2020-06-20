use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use anyhow::{bail, Result};
use tokio::net::TcpStream;
use torut::control::{UnauthenticatedConn, AuthenticatedConn};

const TC_PORT: u16 = 9051;

#[tokio::main]
async fn main() -> Result<()> {
    let sock = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), TC_PORT);
    let stream = TcpStream::connect(sock).await?;

    let mut conn = UnauthenticatedConn::new(stream);
    let info = match conn.load_protocol_info().await {
        Ok(info) => info,
        Err(_) => bail!("failed to load protocol info from Tor")
    };

    let auth_data = info.make_auth_data()?.expect("failed to make auth data");
    if conn.authenticate(&auth_data).await.is_err() {
        bail!("failed to authenticate with Tor")
    }

    let _conn: AuthenticatedConn<TcpStream, ()> = conn.into_authenticated().await;

    Ok(())
}
