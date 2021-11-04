use crate::user_config::{create_config_file, UserConfig};
use crate::user_interface::/*print_help*/ reset_screen;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::RwLock;
use twitch_irc::{
    login::StaticLoginCredentials, /*ClientConfig,*/ SecureTCPTransport, TwitchIRCClient,
};

pub async fn run_command(
    input_buffer: Arc<RwLock<String>>,
    current_channel: Arc<RwLock<String>>,
    config_path: &str,
    client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
) {
    let mut buffer = input_buffer.write().await;
    let mut command = buffer.split_whitespace();

    match command.next() {
        Some(":join") => {
            if let Some(channel) = command.next() {
                join_command(channel.to_string(), current_channel, &client).await;
            }
            buffer.clear();
            reset_screen().await;
        }
        //        Some(":login") => {
        //            if let Some(username) = command.next() {
        //                let oauth_token = command.next();
        //                login_command(username.to_string(), oauth_token.unwrap().to_string(), &client).await;
        //            }
        //        }
        Some(":credentials") => {
            if let Some(username) = command.next() {
                let oauth_token = command.next();
                credentials_command(
                    username.to_string(),
                    oauth_token.unwrap().to_string(),
                    &config_path,
                )
                .await;
            }
            buffer.clear();
            reset_screen().await;
        }
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

// pub async fn login_command(
//     username: String,
//     token: String,
//     client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
// ) {
//
// }

pub async fn credentials_command(new_username: String, token: String, config_path: &str) {
    let config = UserConfig {
        username: new_username,
        oauth_token: token,
    };
    create_config_file(config_path, config).await;
}
