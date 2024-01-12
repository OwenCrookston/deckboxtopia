use shuttle_secrets::SecretStore;
use thiserror::Error;
use url::Url;

#[derive(Debug)]
pub struct Config {
    pub redirect_uri: Url,
}
const REDIRECT_URI: &str = "REDIRECT_URI";

impl Config {
    pub fn new(secret_store: &SecretStore) -> Result<Self, ConfigError> {
        let redirect_uri = secret_store
            .get(REDIRECT_URI)
            .ok_or(ConfigError::Missing(REDIRECT_URI))?;
        Ok(Config {
            redirect_uri: redirect_uri.parse().map_err(anyhow::Error::new)?,
        })
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("missing config {0}")]
    Missing(&'static str),

    #[error("error when parsing {0}")]
    Parse(#[from] anyhow::Error),
}
