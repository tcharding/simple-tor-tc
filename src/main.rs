use std::net::{IpAddr, Ipv4Addr,  SocketAddr};
use anyhow::{bail, Result};
use torut::control::{UnauthenticatedConn};
use torut::onion::{TorSecretKeyV3};

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

    // Start a web server (Hello World: https://github.com/tcharding/rust-web-hello-world)

    //
    // Add an onion service that re-directs to local web server instance
    //

    let key = TorSecretKeyV3::generate();
    ac.add_onion_v3(&key, false, false, false, None, &mut [
                (8000, SocketAddr::new(IpAddr::from(Ipv4Addr::new(127,0,0,1)), 8000)),
            ].iter()).await.unwrap();

    let onion_addr = key.public().get_onion_address();
    println!("onion service available on: {}:8000", onion_addr);

    //
    // Now do a GET request to the web server via the Tor network.
    //

    // curl -x socks5h://127.0.0.1:9050 http://modvw2tdzvbfzm7bffo5ykkzgmk2lirtsiefcbvfcl2d2jx3soplbryd.onion:8000
    // Hello world!

    ::std::thread::park();

    Ok(())
}
