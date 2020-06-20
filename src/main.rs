use anyhow::{bail, Result};

// const TC_PORT: u16 = 9051;

fn main() -> Result<()> {
    assert_can_connect_to_tor()?;

    Ok(())
}

/// Asserts we can make a TCP connection to a local tor instance on TC_PORT.
fn assert_can_connect_to_tor() -> Result<()> {
    // TODO: Connect using tor_control
    // TODO: Connect using tor-client
    // TODO: Connect using torut

    bail!("failed to connect to local Tor instance")
}
