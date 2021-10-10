#![allow(dead_code)]
#![allow(unused_variables)]

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use crate::messages::handle_message;
use crate::terminal::run_terminal;

mod messages;
mod terminal;

#[tokio::main]
pub async fn main() {
    std::thread::spawn(run_terminal);

    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming
    // messages, otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {handle_message(message);}});

    //TODO: Insure proper error handling here.
    //TODO: Create user prompt for channel name.
    // join a channel
    client.join("brandontdev".to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    join_handle.await.unwrap();

}
