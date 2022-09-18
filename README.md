# Nostr types for Rust

- [x] create event 
- [x] serialize event
- [x] deserialize event
- [ ] handle tags
- [ ] subscribe to relays
- [ ] put relays in pools

# Examples

If you want to see what's up - try: 
- `cargo run --example dm`
- `cargo run --example tweet` if you want to see what's up.

**NOTE**: Default behaviour is to run against local relay on port 7000 (see https://github.com/scsibug/nostr-rs-relay)

If you want to try against public relays - change `WS_ENDPOINT` in corresponding examples file
