mod echo;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use anyhow::{bail, Result};
use torut::control::{UnauthenticatedConn};
use torut::onion::{TorSecretKeyV3};
use torut::utils::{run_tor, AutoKillChild};
use tokio::prelude::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    /// Default address to use for the echo server.
    pub static ref ECHO_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8007));
    /// The default TOR socks5 proxy address, `127.0.0.1:9050`.
    pub static ref TOR_PROXY_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9050));
    /// The default TOR Controller Protocol address, `127.0.0.1:9051`.
    pub static ref TOR_CP_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9051));
}

const PORT: u16 = 8007;


#[tokio::main]
async fn main() -> Result<()> {
    // Start Tor
    //   /usr/bin/tor --defaults-torrc /usr/share/tor/tor-service-defaults-torrc -f /etc/tor/torrc
    //
    let child = run_tor("/usr/bin/tor", &mut [
        "--CookieAuthentication", "1",
        "--defaults-torrc", "/usr/share/tor/tor-service-defaults-torrc",
        "-f", "/etc/tor/torrc",
    ].iter()).expect("Starting tor filed");
    let _child = AutoKillChild::new(child);
    println!("Tor instance started");

    //
    // Start an echo server
    //
    tokio::spawn(async move  {
        echo::run(*ECHO_ADDR).await.expect("failed to start echo server")
    });

    //
    // Get an authenticated connection to the Tor via the Tor Controller protocol.
    //

    let stream = simple_tor_tc::connect_tor_cp(*TOR_CP_ADDR).await?;

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

    //
    // Expose an onion service that re-directs to the echo server.
    //

    let key = TorSecretKeyV3::generate();
    ac.add_onion_v3(&key, false, false, false, None, &mut [
                (PORT, SocketAddr::new(IpAddr::from(Ipv4Addr::new(127,0,0,1)), PORT)),
            ].iter()).await.unwrap();

    let onion_addr = key.public().get_onion_address();
    let onion = format!("{}:{}", onion_addr, PORT);
    println!("onion service: {}", onion);

    //
    // Connect to the echo server via the Tor network.
    //

    let mut stream = simple_tor_tc::connect_tor_socks_proxy(*TOR_PROXY_ADDR, onion.as_str()).await?;
    println!("TorStream connection established");

    println!("writing 'ping' to the stream");
    stream.write_all(b"ping\n").await?;

    println!("reading from the stream ...");
    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf).await?;
    println!("received {} bytes: {}", n, std::str::from_utf8(&buf[0..n]).unwrap());

    Ok(())
}
