# Nostr types for Rust

- [x] create event 
- [x] serialize event
- [x] deserialize event
- [ ] handle tags
- [ ] subscribe to relays
- [ ] put relays in pools

# Examples

Examples can be run as follows:

- `cargo run --example dm`
- `cargo run --example tweet`

**NOTE**: Default behaviour is to run against local relay on port 7000 (see https://github.com/scsibug/nostr-rs-relay)

If you want to try against public relays, change `WS_ENDPOINT` in the corresponding examples files.
