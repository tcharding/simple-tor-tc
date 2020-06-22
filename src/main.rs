use anyhow::{bail, Result};
use tokio::net::TcpStream;
use torut::control::{UnauthenticatedConn, AuthenticatedConn};

#[tokio::main]
async fn main() -> Result<()> {
    let stream = simple_tor_tc::connect().await?;
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
