#![allow(dead_code, unused_variables, unreachable_code, unused_must_use)]

use crate::cli::Cli;
use crate::messages::format_message;
use crate::messages::print_message;
use core::time;
use dotenv;
use std::io;
use std::io::Write;
use std::thread;
use structopt::StructOpt;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tokio::{select, sync::broadcast};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

mod cli;
mod messages;

#[tokio::main]
pub async fn main() {
    // Load env file constants.
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

    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
    let shutdown_rx2 = shutdown_tx.subscribe();

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // start consuming incoming messages, otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        loop {
            select! {
                Some(message) = incoming_messages.recv() => {
                    print_message(format_message(message));
                },
                // End process if sender message received.
                _ = shutdown_rx.recv() => break,
            };
        }
    });

    let join_handle2 = tokio::spawn(async move {
        // Set terminal to raw mode to allow reading
        // stdin one key at a time.
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        // Use asynchronous stdin.
        let mut stdin = termion::async_stdin().keys();

        loop {
            // Read input (if any)
            let input = stdin.next();

            // If a key was pressed
            if let Some(Ok(key)) = input {
                match key {
                    // TODO: Hook ctrl-c for "proper" shutdown.
                    // Exit if 'Esc' was pressed:
                    termion::event::Key::Esc => {
                        // Send message to receivers to end process.
                        shutdown_tx.send(()).ok();
                        break;
                    },

                    // Else, print the pressed key:
                    _ => {
                        // TODO: need a match here for properly formatted imput.
                        write!(stdout, "{:?}", key).unwrap();
                        stdout.lock().flush().unwrap();
                    }
                }
            }
            thread::sleep(time::Duration::from_millis(50));
        }
    });

    // TODO: Add another tokio task for ctrl-c handling.

    // TODO: Need error-handling for channels
    // that do not exist and incorrect user input.

    // Join channel chat from argument string:
    client.join(channel_name);

    // keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    futures::join!(join_handle, join_handle2);
}
