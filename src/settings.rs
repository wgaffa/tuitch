use std::{io::stdout, io::Write, sync::Arc, collections::HashSet};
use termion::terminal_size;
use tokio::sync::RwLock;
use twitch_irc::{login::StaticLoginCredentials, SecureTCPTransport, TwitchIRCClient};

pub async fn join_command(
    input_buffer: Arc<RwLock<String>>,
    client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) {
    let mut channel_hash = HashSet::<String>::new();
    let mut buffer = input_buffer.write().await;
    let channel = buffer.strip_prefix(":join ");

    channel_hash.insert(channel.unwrap().to_string());
    client.set_wanted_channels(channel_hash);
    buffer.clear();

    let (x, y) = terminal_size().unwrap();
    print!("{}", termion::clear::All);
    print!("{}> ", termion::cursor::Goto(1, y));
    stdout().lock().flush();
}
