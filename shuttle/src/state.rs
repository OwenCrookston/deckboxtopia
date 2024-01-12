use std::{ops::Deref, sync::Arc};

use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use tracing::info;

use crate::{
    config::{Config, ConfigError},
    discord::{self, DiscordApi},
};

/// Defines the Api State that can contain things like persistance, db, env vars, etc.
///
/// It is behind an AtomicReferenceCounter so it is safe to clone between threads and points to the
/// shared memory space.
#[derive(Clone)]
pub struct ApiState(Arc<ApiStateInner>);

pub struct ApiStateInner {
    pub persist: PersistInstance,
    pub config: Config,
    pub discord_api: DiscordApi,
}

impl ApiState {
    pub fn new(persist: PersistInstance, secret_store: &SecretStore) -> Result<Self, ConfigError> {
        let config = Config::new(secret_store)?;
        info!(message = "Loaded config", cfg = ?config);
        let client = surf::Client::new();
        let discord_config = discord::Config::new(secret_store)?;
        let discord_api = DiscordApi::new(client, discord_config);

        Ok(ApiState(Arc::new(ApiStateInner {
            persist,
            config,
            discord_api,
        })))
    }
}

impl Deref for ApiState {
    type Target = ApiStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
