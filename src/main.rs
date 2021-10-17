#![allow(dead_code, unused_variables, unreachable_code)]

use dotenv;
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
    dotenv::dotenv().ok();

    // take command-line arguments for user and channel names.
    let args = Cli::from_args();
    let channel_name: String = args.channel;
    let login_name: String = args.user;

    // Login with CLI argument username.
    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        login_name,
        Some(dotenv::var("OAUTH_TOKEN").unwrap()),
    ));

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
    client.join(channel_name);

    // keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    join_handle.await.unwrap();
}
