# Simple Tor TC (Tor Controller)

Simple Rust application to demonstrate interfacing with a local Tor instance via the Tor Controller protocol

### TODO

1. Act as a client:
   - Connect to an onion service i.e., do a HTTP request.

2. Act as a server:
   - Expose a hidden service
   - Start a simple web server
   - Test using Tor browser


### potential libraries to use:

- [tor_client](https://github.com/resolvingarchitecture/tor-client)
- [torut](https://github.com/teawithsand/torut)
- [tor_control](https://crates.io/crates/tor_control)
