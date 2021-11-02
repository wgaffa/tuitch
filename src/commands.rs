use crate::user_interface::reset_screen;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::RwLock;
use twitch_irc::{
    login::StaticLoginCredentials, /*ClientConfig,*/ SecureTCPTransport, TwitchIRCClient,
};

pub async fn run_command(
    input_buffer: Arc<RwLock<String>>,
    current_channel: Arc<RwLock<String>>,
    client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) {
    let mut buffer = input_buffer.write().await;
    let mut command = buffer.split_whitespace();

    match command.next() {
        Some(":join") => {
            if let Some(channel) = command.next() {
                join_command(channel.to_string(), current_channel, &client).await;
            }
            // TODO: Maybe abstract this?
            buffer.clear();
            reset_screen().await;
        }
        //        Some(":login") => if let Some(username) = command.next() {},
        _ => {}
    }
}

pub async fn join_command(
    channel: String,
    current_channel: Arc<RwLock<String>>,
    client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) {
    let mut channel_hash = HashSet::<String>::new();
    let mut channel_buffer = current_channel.write().await;
    channel_buffer.clear();
    channel_buffer.push_str(&channel);
    channel_hash.insert(channel);
    client.set_wanted_channels(channel_hash);
}
