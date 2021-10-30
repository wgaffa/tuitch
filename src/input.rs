use std::{io::Stdout, io::Write, sync::Arc};
use termion::raw::RawTerminal;
use tokio::sync::RwLock;
use twitch_irc::{login::StaticLoginCredentials, SecureTCPTransport, TwitchIRCClient};

// Print the pressed key and add the input to the input_buffer.
pub async fn write_input(
    user_input: char,
    input_buffer: Arc<RwLock<String>>,
    stdout: Arc<RwLock<RawTerminal<Stdout>>>,
) {
    write!(stdout.write().await, "{}", user_input);
    input_buffer.write().await.push(user_input);
    stdout.write().await.lock().flush().unwrap();
}

// On 'Backspace'
// Remove the last element from the input_buffer,
// move the cursor one column to the left,
// clear all items after the cursor.
//
// If the input_buffer is empty, backspace
// does nothing.
pub async fn delete_input(
    input_buffer: Arc<RwLock<String>>,
    stdout: Arc<RwLock<RawTerminal<Stdout>>>,
) {
    if !input_buffer.read().await.is_empty() {
        input_buffer.write().await.pop();
        write!(
            stdout.write().await,
            "{}{}",
            termion::cursor::Left(1),
            termion::clear::AfterCursor
        );
        stdout.write().await.lock().flush().unwrap();
    }
}

// Send typed user input when 'Enter' key is pressed.
// Clear the input_buffer, clear the current line,
// and return the cursor to the first column.
pub async fn send_message(
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    channel_name: Arc<RwLock<String>>,
    user_name: Arc<RwLock<String>>,
    input_buffer: Arc<RwLock<String>>,
    stdout: Arc<RwLock<RawTerminal<Stdout>>>,
) {
    client
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
    stdout.write().await.lock().flush().unwrap();
}
