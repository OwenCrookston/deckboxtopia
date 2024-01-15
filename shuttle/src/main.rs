#![cfg_attr(debug_assertions, allow(dead_code))]
mod auth;
mod config;
mod discord;
mod endpoints;
mod models;
mod state;
mod utils;

use axum::http::Method;
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use state::ApiState;
use tower_http::cors::{Any, CorsLayer};
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

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let router = endpoints::routes(state)
        .layer(cors)
        .fallback_service(ServeDir::new("static"));

    Ok(router.into())
}
