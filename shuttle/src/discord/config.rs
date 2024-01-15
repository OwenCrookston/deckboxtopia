use std::fmt::Debug;

use shuttle_secrets::SecretStore;

use crate::config::ConfigError;

const DISCORD_APP_CLIENT_ID: &str = "DISCORD_APP_CLIENT_ID";
const DISCORD_APP_CLIENT_SECRET: &str = "DISCORD_APP_CLIENT_SECRET";
const DISCORD_APP_REDIRECT_URI: &str = "DISCORD_APP_REDIRECT_URI";
const DISCORD_API_ENDPOINT: &str = "DISCORD_API_ENDPOINT";

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub api_endpoint: String,
    pub redirect_uri: String,
}

impl Config {
    pub fn new(secret_store: &SecretStore) -> Result<Self, ConfigError> {
        let client_id = secret_store
            .get(DISCORD_APP_CLIENT_ID)
            .ok_or(ConfigError::Missing(DISCORD_APP_CLIENT_ID))?;
        let client_secret = secret_store
            .get(DISCORD_APP_CLIENT_SECRET)
            .ok_or(ConfigError::Missing(DISCORD_APP_CLIENT_SECRET))?;
        let api_endpoint = secret_store
            .get(DISCORD_API_ENDPOINT)
            .ok_or(ConfigError::Missing(DISCORD_API_ENDPOINT))?;
        let redirect_uri = secret_store
            .get(DISCORD_APP_REDIRECT_URI)
            .ok_or(ConfigError::Missing(DISCORD_APP_REDIRECT_URI))?;
        Ok(Config {
            client_id,
            client_secret,
            api_endpoint,
            redirect_uri,
        })
    }
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiscordConfig")
            .field("client_id", &self.client_id)
            .field("client_secret", &"obfuscated".to_string())
            .field("api_endpoint", &self.api_endpoint)
            .finish()
    }
}
