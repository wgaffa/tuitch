#![allow(unused_variables, unused_must_use)]

// use crate::input::{delete_input, send_message, write_input};
use crate::messages::{format_message, print_message};
use crate::settings::join_command;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{io::stdout, io::Write, sync::Arc};
use termion::{input::TermRead, raw::IntoRawMode, screen::AlternateScreen, terminal_size};
use tokio::{select, sync::broadcast, sync::RwLock};
use toml;
use twitch_irc::{
    login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

mod input;
mod messages;
mod settings;

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    username: String,
    oauth_token: String,
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    // TODO: Add another tokio task for ctrl-c handling.
    // TODO: Need error-handling for channels
    // that do not exist and incorrect user input.
    // TODO: Possible tabs for multiple chats?

    // Create alternate screen, restores terminal on drop.
    let screen = AlternateScreen::from(stdout());

    // TODO: consider abstraction?
    let (x, y) = terminal_size().unwrap();
    print!("{}", termion::clear::All);
    print!("{}> ", termion::cursor::Goto(1, y));
    stdout().lock().flush();

    let user_config = UserConfig {
        username: String::new(),
        oauth_token: String::new(),
    };

    let user_config_toml = toml::to_string(&user_config).unwrap();
    fs::write("Config.toml", user_config_toml)?;

    let channel_name = Arc::new(RwLock::new(String::new()));
    let user_name = Arc::new(RwLock::new("brandont".to_string()));
    let channel_name_read = Arc::clone(&channel_name);
    let user_name_read = Arc::clone(&user_name);

    // Input-buffer for user's typed input and chat messages.
    // This is a shared state to allow proper handling with incoming
    // server messages while unsent user input is in the console.
    let input_buffer_lock = Arc::new(RwLock::new(String::new()));
    let input_buffer = Arc::clone(&input_buffer_lock);
    let input_buffer2 = Arc::clone(&input_buffer_lock);

    let config = ClientConfig::default();

    //    if !user_config.oauth_token.is_empty() {
    //        let config = ClientConfig::new_simple(StaticLoginCredentials::new(
    //            user_name.read().await.to_string(),
    //            Some(user_config.oauth_token),
    //        ));
    //    }

    // Create tx/rx to send and receive shutdown signal
    // when specific user input is detected.
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
    let shutdown_rx2 = shutdown_tx.subscribe();
    let shutdown_rx3 = shutdown_tx.subscribe();
    let (command_tx, command_rx) = broadcast::channel(2);
    let mut command_rx = command_tx.subscribe();

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // TwitchIRCClient is thread safe, clone() can be called here.
    let client2 = client.clone();

    // Start consuming incoming messages, otherwise they will back up.
    // First tokio task to listen for incoming server messages
    // and format them as needed before printing them to the console.
    let join_handle = tokio::spawn(async move {
        loop {
            select! {
                Some(message) = incoming_messages.recv() => {
                   print_message(format_message(message), input_buffer2.read().await.to_string());
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

            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Esc => {
                        // Send message to receivers to end process.
                        shutdown_tx.send(()).ok();
                        break;
                    }

                    termion::event::Key::Char('\n') => {
                        //                        send_message(
                        //                            client2.clone(),
                        //                            Arc::clone(&channel_name_read),
                        //                            Arc::clone(&user_name_read),
                        //                            Arc::clone(&input_buffer_lock),
                        //                            Arc::clone(&stdout),
                        //                        );
                        
                        let first_char = input_buffer.read().await.chars().nth(0);
                        if first_char == Some(':') {
                            command_tx.send(()).ok();
                        }

                        if !input_buffer.read().await.is_empty() {
                            client2
                                .privmsg(
                                    channel_name.read().await.to_string().to_owned(),
                                    input_buffer.read().await.to_owned(),
                                )
                                .await
                                .unwrap();

                            print!(
                                " {}\r{}: {}\n\n",
                                termion::clear::CurrentLine,
                                user_name.read().await.to_string(),
                                input_buffer.read().await.to_string()
                            );

                            input_buffer.write().await.clear();
                            print!("{}\r> ", termion::clear::CurrentLine);
                            stdout.lock().flush().unwrap();
                        }
                    }

                    termion::event::Key::Char(user_input) => {
                        //                        write_input(
                        //                            user_input,
                        //                           Arc::clone(&input_buffer_lock),
                        //                            Arc::clone(&stdout),
                        //                        );
                        write!(stdout, "{}", user_input);
                        input_buffer.write().await.push(user_input);
                        stdout.lock().flush().unwrap();
                    }

                    termion::event::Key::Backspace => {
                        //                        delete_input(Arc::clone(&input_buffer_lock), Arc::clone(&stdout));
                        if !input_buffer.read().await.is_empty() {
                            input_buffer.write().await.pop();
                            write!(
                                stdout,
                                "{}{}",
                                termion::cursor::Left(1),
                                termion::clear::AfterCursor
                            );
                            stdout.lock().flush().unwrap();
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    let join_handle3 = tokio::spawn(async move {
        loop {
            select! {
                Ok(command) = command_rx.recv() => join_command(Arc::clone(&input_buffer_lock), &client).await
            };
        }
    });

    // Keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    futures::join!(join_handle, join_handle2, join_handle3);
    Ok(())
}
