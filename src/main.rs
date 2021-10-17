#![allow(dead_code, unused_variables)]

use crate::cli::Cli;
use crate::messages::handle_message;
use structopt::StructOpt;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

mod cli;
mod messages;

#[tokio::main]
pub async fn main() {
    // take command-line arguments for channel name.
    // TODO: Will also take in user name when user authentication
    // is implimented.
    let args = Cli::from_args();
    let channel_name: String = args.channel;

    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // start consuming incoming messages, otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            handle_message(message);
        }
    });

    // TODO: Need error-handling for channels
    // that do not exist and incorrect user input.

    // Join channel chat from argument string:
    client.join(channel_name.to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    join_handle.await.unwrap();
}
