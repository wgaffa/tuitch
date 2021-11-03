#![allow(unused_variables, unused_must_use)]

use crate::commands::run_command;
use crate::messages::{format_message, print_message, print_user_message};
use crate::user_config::{get_client_config, set_client_config};
use crate::user_interface::reset_screen;
use std::{io::stdout, io::Write, sync::Arc};
use termion::{input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tokio::{select, sync::broadcast, sync::RwLock};
use twitch_irc::{login::StaticLoginCredentials, SecureTCPTransport, TwitchIRCClient};

mod commands;
mod messages;
mod user_config;
mod user_interface;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    // TODO: Add another tokio task for ctrl-c handling.
    // TODO: Need error-handling for channels
    // that do not exist and incorrect user input.

    let config_path = "Config.toml";
    let user_config = get_client_config(config_path).await;

    let current_channel = Arc::new(RwLock::new(String::new()));
    let user_name = Arc::new(RwLock::new(user_config.username));
    let current_channel_read = Arc::clone(&current_channel);
    let user_name_read = Arc::clone(&user_name);

    // Input-buffer for user's typed input and chat messages.
    // This is a shared state to allow proper handling with incoming
    // server messages while unsent user input is in the console.
    let input_buffer_lock = Arc::new(RwLock::new(String::new()));
    let input_buffer = Arc::clone(&input_buffer_lock);
    let input_buffer2 = Arc::clone(&input_buffer_lock);

    // Create tx/rx to send and receive shutdown signal
    // when specific user input is detected.
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
    let shutdown_rx2 = shutdown_tx.subscribe();
    let shutdown_rx3 = shutdown_tx.subscribe();

    // Channel for chat-line commands and settings.
    let (command_tx, command_rx) = broadcast::channel(2);
    let mut command_rx = command_tx.subscribe();

    // The TwitchIRCClient is built with either the default (read-only) or Twitch
    // login credentials (username & OAuth token pair).
    let (mut incoming_messages, client) = TwitchIRCClient::<
        SecureTCPTransport,
        StaticLoginCredentials,
    >::new(set_client_config(config_path));

    // TwitchIRCClient is thread safe, clone() can be called here.
    // client2 is used to send user messages to the Twitch servers.
    let client2 = client.clone();

    // Create alternate screen, restores terminal on drop.
    let screen = AlternateScreen::from(stdout());
    reset_screen().await;

    // Start consuming incoming messages, otherwise they will back up.
    //
    // First tokio task to listen for incoming server messages
    // and format them as needed before printing them to the console.
    let join_handle = tokio::spawn(async move {
        loop {
            select! {
                Some(message) = incoming_messages.recv() => {
                   print_message(format_message(message).await, input_buffer2.read().await.to_string()).await;
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
        let mut stdout = stdout().into_raw_mode().unwrap();

        // Use asynchronous stdin.
        let mut stdin = termion::async_stdin().keys();

        loop {
            let input = stdin.next();
            let first_char = input_buffer.read().await.chars().nth(0);
            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Esc => {
                        // Send message to receivers to end process.
                        shutdown_tx.send(()).ok();
                        break;
                    }

                    termion::event::Key::Char('\n') => {
                        if !input_buffer.read().await.is_empty() {
                            if first_char == Some(':') {
                                // If the entered input buffer starts with a ':'
                                // then the run_command function is executed,
                                // parsing the command and running its logic.
                                command_tx.send(()).ok();
                            } else {
                                // if the input_buffer does not begin with a ':',
                                // it's treated as a normal chat message, which is 
                                // sent to the Twitch servers.
                                client2
                                    .privmsg(
                                        current_channel.read().await.to_owned(),
                                        input_buffer.read().await.to_owned(),
                                    )
                                    .await
                                    .unwrap();
                                // Sent messages have to be formatted and printed
                                // to the terminal manually, twitch_irc doesn't 
                                // seem to see these as incoming messages.
                                print_user_message(
                                    user_name.read().await.as_str(),
                                    input_buffer.read().await.to_string(),
                                )
                                .await;
                                input_buffer.write().await.clear();
                                print!("{}\r> ", termion::clear::CurrentLine);
                            }
                        }
                    }
                    termion::event::Key::Char(user_input) => {
                        // write user input to the console
                        // and save it to input_buffer
                        write!(stdout, "{}", user_input);
                        input_buffer.write().await.push(user_input);
                    }

                    termion::event::Key::Backspace => {
                        if !input_buffer.read().await.is_empty() {
                            input_buffer.write().await.pop();
                            write!(
                                stdout,
                                "{}{}",
                                termion::cursor::Left(1),
                                termion::clear::AfterCursor
                            );
                        }
                    }
                    _ => {}
                }
                stdout.lock().flush().unwrap();
            }
        }
    });

    let join_handle3 = tokio::spawn(async move {
        loop {
            select! {
                // if a command ':' is found in a sent input buffer,
                // call run_command to parse the input and handle the command
                Ok(command) = command_rx.recv() => run_command(
                    Arc::clone(&input_buffer_lock),
                    Arc::clone(&current_channel_read),
                    &client
                    ).await
            };
        }
    });

    // Keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    futures::join!(join_handle, join_handle2, join_handle3);
    Ok(())
}
