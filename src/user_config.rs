use serde::{Deserialize, Serialize};
use std::fs;
use twitch_irc::{login::StaticLoginCredentials, ClientConfig};

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    pub username: String,
    pub oauth_token: String,
}

pub async fn set_client_config(path: &str) -> ClientConfig<StaticLoginCredentials> {
    if fs::metadata(path).is_ok() {
        let config_file_content = fs::read_to_string(path).unwrap();
        let config: UserConfig = toml::from_str(&config_file_content.as_str()).unwrap();
        ClientConfig::new_simple(StaticLoginCredentials::new(
            config.username,
            Some(config.oauth_token),
        ))
    } else {
        let config = UserConfig {
            username: String::new(),
            oauth_token: String::new(),
        };
        create_config_file(path, config).await.unwrap();
        ClientConfig::default()
    }
}

pub async fn create_config_file(path: &str, config: UserConfig) -> std::io::Result<()> {
    let config_toml = toml::to_string(&config).unwrap();
    fs::write(&path, config_toml)?;
    Ok(())
}

pub async fn get_client_config(path: &str) -> UserConfig {
    let config_file_content = fs::read_to_string(path).unwrap();
    let config: UserConfig = toml::from_str(&config_file_content.as_str()).unwrap();

    return config;
}
