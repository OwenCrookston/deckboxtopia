use std::{collections::HashMap, ops::Deref, sync::Arc};

use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use tracing::info;

use crate::{
    auth::session::Session,
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
    pub cipher: Aes256Gcm,
}

impl ApiState {
    pub fn new(persist: PersistInstance, secret_store: &SecretStore) -> Result<Self, ConfigError> {
        let config = Config::new(secret_store)?;
        info!(message = "Loaded config", cfg = ?config);
        let client = surf::Client::new();
        let discord_config = discord::Config::new(secret_store)?;
        let discord_api = DiscordApi::new(client, discord_config);

        // WARNING: Key is produced randomly on startup. It is currently not persisted!
        // This will log out all users on startup
        let key = Aes256Gcm::generate_key(OsRng);
        let cipher = Aes256Gcm::new(&key);

        Ok(ApiState(Arc::new(ApiStateInner {
            persist,
            config,
            discord_api,
            cipher,
        })))
    }

    pub fn save_session(&self, sess: &Session, nonce: Vec<u8>) -> anyhow::Result<()> {
        self.persist.save(&sess.get_kid().to_string(), nonce)?;
        Ok(())
    }

    pub fn get_session(&self, sess: &str) -> anyhow::Result<Option<Vec<u8>>> {
        let mut sessions: Sessions = self
            .persist
            .load::<Option<_>>("sessions")?
            .unwrap_or_default();
        Ok(sessions.remove(sess))
    }
}

pub type Sessions = HashMap<String, Vec<u8>>;

impl Deref for ApiState {
    type Target = ApiStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
