mod echo;

use torut::utils::{run_tor, AutoKillChild};
use std::io::prelude::*;
use tor_stream::TorStream;
use std::net::{IpAddr, Ipv4Addr,  SocketAddr};
use anyhow::{bail, Result};
use torut::control::{UnauthenticatedConn};
use torut::onion::{TorSecretKeyV3};

const PORT: u16 = 8007;

#[tokio::main]
async fn main() -> Result<()> {
    //
    // Start Tor
    // /usr/bin/tor --defaults-torrc /usr/share/tor/tor-service-defaults-torrc -f /etc/tor/torrc
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
        let addr = socket_addr();
        echo::run(addr).await.expect("failed to start echo server")
    });

    //
    // Get an authenticated connection to the Tor via the Tor Controller protocol.
    //

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

    let mut stream = TorStream::connect(onion.as_str()).expect("Failed to connect");
    println!("TorStream connection established");

    println!("writing 'ping' to the stream");
    stream.write_all(b"ping\n").expect("Failed to send request");

    let mut stream = stream.unwrap();

    println!("reading from the stream ...");
    let mut buf = [0u8; 128];
    let n = stream.read(&mut buf)?;
    println!("received {} bytes: {}", n, std::str::from_utf8(&buf[0..n]).unwrap());

    Ok(())
}

fn socket_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::from(Ipv4Addr::new(127,0,0,1)), PORT)
}
