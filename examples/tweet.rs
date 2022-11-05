use std::error::Error;
use std::str::FromStr;
use secp256k1::SecretKey;

use nostr::{ClientMessage, Event, Keys, RelayMessage, SubscriptionFilter};
use tungstenite::{connect, Message as WsMessage};
use url::Url;
use nostr::event::KindBase;

const ALICE_SK: &str = "6b911fd37cdf5c81d4c0adb1ab7fa822ed253ab0ad9aa18d77257c88b29b718e";
const BOB_SK: &str = "7b911fd37cdf5c81d4c0adb1ab7fa822ed253ab0ad9aa18d77257c88b29b718e";
// const WS_ENDPOINT: &str = "wss://relayer.fiatjaf.com/";
// const WS_ENDPOINT: &str = "wss://nostr-relay-dev.wlvs.space";
const WS_ENDPOINT: &str = "ws://localhost:7000";


fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let (mut socket, response) = connect(Url::parse(WS_ENDPOINT)?).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let alice_keys = Keys::new(SecretKey::from_str(ALICE_SK)?)?;

    let bob_keys = Keys::new(SecretKey::from_str(BOB_SK)?)?;

    let alice_says_hi =
        ClientMessage::new_event(Event::new_textnote("hi from alice", &alice_keys, &vec![])?);
    let bob_says_hi = ClientMessage::new_event(Event::new_textnote("bob says hello", &bob_keys, &vec![])?);

    let subscribe_to_alice = ClientMessage::new_req(
        "abcdefgh",
        vec![SubscriptionFilter::new()
            .authors(vec![alice_keys.public_key])
            .kind_base(KindBase::TextNote)],
    );

    let subscribe_to_bob = ClientMessage::new_req(
        "1234567",
        vec![SubscriptionFilter::new()
            .authors(vec![bob_keys.public_key])
            .kind_base(KindBase::TextNote)],
    );

    socket.write_message(WsMessage::Text(subscribe_to_alice.to_json()))?;
    socket.write_message(WsMessage::Text(subscribe_to_bob.to_json()))?;

    socket.write_message(WsMessage::Text(alice_says_hi.to_json()))?;
    socket.write_message(WsMessage::Text(bob_says_hi.to_json()))?;

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg_text = msg.to_text().expect("Failed to conver message to text");
        if let Ok(handled_message) = RelayMessage::from_json(msg_text) {
            match handled_message {
                RelayMessage::Empty => {
                    println!("Empty message")
                }
                RelayMessage::Notice { message } => {
                    println!("Got a notice: {}", message);
                }
                RelayMessage::Event { event, subscription_id } => {
                    println!("Got an event on subscription {subscription_id} with content: {}", event.content);
                }
                RelayMessage::EndOfStoredEvents { subscription_id } => {
                    println!("Relay signalled End of Stored Events, for subscription {subscription_id}");
                }
            }
        }
        else {
            println!("Got unexpected message: {}", msg_text);
        }
    }
}
