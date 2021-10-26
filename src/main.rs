#![allow(
    dead_code,
    unused_variables,
    unreachable_code,
    unused_must_use,
    path_statements
)]

use crate::cli::Cli;
use crate::messages::format_message;
use crate::messages::print_message;
use core::time;
use std::io::stdout;
use dotenv;
use std::io;
use std::io::Write;
use std::thread;
use structopt::StructOpt;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tokio::{select, sync::broadcast};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

mod cli;
mod messages;

#[tokio::main]
pub async fn main() {
    // Create alternate screen, restores terminal on drop.
    let screen = AlternateScreen::from(stdout());

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

    // Create tx/rx to send and receive shutdown signal
    // when specific user input is detected.
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
    let shutdown_rx2 = shutdown_tx.subscribe();

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // TODO: This is bad, need to change. But it works.
    let client2 = client.clone();
    let channel_name2 = channel_name.clone();

    // start consuming incoming messages, otherwise they will back up.
    // First tokio task to listen for incoming server messages.
    let join_handle = tokio::spawn(async move {
        print!("{}", termion::cursor::Goto(1, 1));
        loop {
            select! {
                Some(message) = incoming_messages.recv() => {
                    // TODO: Once the input_buffer is accessible to this 
                    // task, it needs to be passed in as the second
                    // argument to the print_message fn.
                    print_message(format_message(message));
                },
                // End process if sender message received.
                _ = shutdown_rx.recv() => break,
            };
        }
    });

    // Second tokio task to listen to user input and outgoing chat messages.
    let join_handle2 = tokio::spawn(async move {
        // Set terminal to raw mode to allow reading
        // stdin one key at a time.
        let mut stdout = io::stdout().into_raw_mode().unwrap();

        // Use asynchronous stdin.
        let mut stdin = termion::async_stdin().keys();

        // Input buffer; save user input per keystroke.
        // TODO: This variable needs to be accessible to
        // both tokio tasks.
        let mut input_buffer = String::new();

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
                    }

                    // Send typed user input when 'Enter' key is pressed.
                    termion::event::Key::Char('\n') => {
                        client2
                            .privmsg(channel_name2.to_owned(), input_buffer.to_owned())
                            .await
                            .unwrap();

                        // Print user input to the chat feed.
                        print!("{}\r[You]: {}\n", termion::clear::CurrentLine, input_buffer);

                        // Clear the input_buffer, clear the current line,
                        // and call the carriage return ANSI escape
                        // to return the cursor to the first column of the line.
                        input_buffer.clear();
                        write!(stdout, "{}\r> ", termion::clear::CurrentLine);
                        stdout.lock().flush().unwrap();
                    }

                    // Else, print the pressed key and add the input
                    // to the input_buffer.
                    termion::event::Key::Char(user_input) => {
                        print!("{}", user_input);
                        input_buffer.push(user_input);
                        stdout.lock().flush().unwrap();
                    }

                    // On 'Backspace'
                    // Remove the last element from the input_buffer,
                    // move the cursor one column to the left,
                    // call ANSI escape sequence to clear from the cursor
                    // to the end of the line.
                    termion::event::Key::Backspace => {
                        input_buffer.pop();
                        write!(stdout, "{}{}",
                            termion::cursor::Left(1),
                            termion::clear::AfterCursor);
                        stdout.lock().flush().unwrap();
                    }
                    _ => {}
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
