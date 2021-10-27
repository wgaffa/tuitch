#![allow(unused_variables, unused_must_use)]

use crate::cli::Cli;
use crate::messages::{format_message, print_message};
use std::{io, io::stdout, io::Write, sync::Arc};
use structopt::StructOpt;
use termion::{input::TermRead, raw::IntoRawMode, screen::AlternateScreen, terminal_size};
use tokio::{select, sync::broadcast, sync::RwLock};
use twitch_irc::{
    login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

mod cli;
mod messages;

#[tokio::main]
pub async fn main() {
    // TODO: Login credential input.
    // TODO: Add another tokio task for ctrl-c handling.
    // TODO: Need error-handling for channels
    // that do not exist and incorrect user input.
    // TODO: Possible tabs for multiple chats?
    //
    // Create alternate screen, restores terminal on drop.
    let screen = AlternateScreen::from(stdout());
    let (x, y) = terminal_size().unwrap();
    print!("{}", termion::cursor::Goto(1, y));

    // Load env file constants.
    dotenv::dotenv().ok();

    // Take command-line arguments for user and channel names.
    // Use RwLock to allow shared state.
    //
    // TODO: Convert this functionality from CLI to fit within
    // the TUI. More streamlined user login and channel joining.
    let args = Cli::from_args();
    let channel_name = Arc::new(RwLock::new(args.channel));
    let login_name = Arc::new(RwLock::new(args.user));
    let channel_name_read = Arc::clone(&channel_name);
    let login_name_read = Arc::clone(&login_name);

    // Input-buffer for user's typed input and chat messages.
    // This is a shared state to allow proper handling with incoming
    // server messages while user input is in the console.
    let input_buffer_lock = Arc::new(RwLock::new(String::new()));
    let input_buffer = Arc::clone(&input_buffer_lock);
    let input_buffer2 = Arc::clone(&input_buffer_lock);

    // Login with CLI argument username.
    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        login_name.read().await.to_string(),
        Some(dotenv::var("OAUTH_TOKEN").unwrap()),
    ));

    // Create tx/rx to send and receive shutdown signal
    // when specific user input is detected.
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
    let shutdown_rx2 = shutdown_tx.subscribe();

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
        let mut stdout = io::stdout().into_raw_mode().unwrap();

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

                    // Send typed user input when 'Enter' key is pressed.
                    // Clear the input_buffer, clear the current line,
                    // and return the cursor to the first column.
                    termion::event::Key::Char('\n') => {
                        client2
                            .privmsg(
                                channel_name_read.read().await.to_string().to_owned(),
                                input_buffer.read().await.to_owned(),
                            )
                            .await
                            .unwrap();

                        print!(
                            "{}\r[{}]: {}\n",
                            termion::clear::CurrentLine,
                            login_name_read.read().await.to_string(),
                            input_buffer.read().await.to_string()
                        );

                        input_buffer.write().await.clear();
                        write!(stdout, "{}\r> ", termion::clear::CurrentLine);
                        stdout.lock().flush().unwrap();
                    }

                    // Else, print the pressed key and add the input
                    // to the input_buffer.
                    termion::event::Key::Char(user_input) => {
                        print!("{}", user_input);
                        input_buffer.write().await.push(user_input);
                        stdout.lock().flush().unwrap();
                    }

                    // On 'Backspace'
                    // Remove the last element from the input_buffer,
                    // move the cursor one column to the left,
                    // clear all items after the cursor.
                    //
                    // If the input_buffer is empty, backspace
                    // does nothing.
                    termion::event::Key::Backspace => {
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

    client.join(channel_name.read().await.to_string());

    // Keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    futures::join!(join_handle, join_handle2);
}
