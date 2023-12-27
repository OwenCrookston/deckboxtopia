#![cfg_attr(debug_assertions, allow(dead_code))]
mod endpoints;
mod models;
mod state;

use shuttle_persist::PersistInstance;
use state::ApiState;
use tower_http::services::ServeDir;
use tracing::warn;

#[shuttle_runtime::main]
async fn main(#[shuttle_persist::Persist] persist: PersistInstance) -> shuttle_axum::ShuttleAxum {
    if let Err(err) = persist.clear() {
        warn!("Error clearing persistance: {:?}", err);
    }

    let state = ApiState::new(persist);

    let router = endpoints::routes(state).fallback_service(ServeDir::new("static"));

    Ok(router.into())
}
