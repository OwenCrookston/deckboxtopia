#![cfg_attr(debug_assertions, allow(dead_code))]
mod auth;
mod config;
mod discord;
mod endpoints;
mod models;
mod state;
mod utils;

use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use state::ApiState;
use tower_http::services::ServeDir;
use tracing::warn;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    if let Err(err) = persist.clear() {
        warn!("Error clearing persistance: {:?}", err);
    }
    let state = ApiState::new(persist, &secret_store).map_err(anyhow::Error::new)?;

    let router = endpoints::routes(state).fallback_service(ServeDir::new("static"));

    Ok(router.into())
}
