use anyhow::{bail, Result};
use torut::control::{UnauthenticatedConn};

#[tokio::main]
async fn main() -> Result<()> {
    let stream = simple_tor_tc::connect().await?;

    let mut utc = UnauthenticatedConn::new(stream);

    let info = match utc.load_protocol_info().await {
        Ok(info) => info,
        Err(_) => bail!("failed to load protocol info from Tor")
    };
    let ad = info.make_auth_data()?.expect("failed to make auth data");

    if utc.authenticate(&ad).await.is_err() {
        bail!("failed to authenticate with Tor")
    }
    let mut ac = utc.into_authenticated().await;

    ac.set_async_event_handler(Some(|_| {
        async move { Ok(()) }
    }));

    ac.take_ownership().await.unwrap();

    Ok(())
}
