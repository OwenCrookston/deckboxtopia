use axum::{body::Body, routing::post, Router};
use shuttle_persist::PersistInstance;

pub mod create_library;
mod update_library;

use create_library::create_library;
use update_library::update_library;

pub fn routes() -> Router<PersistInstance, Body> {
    Router::new()
        .route("/create", post(create_library))
        .route("/update/:library_id", post(update_library))
}
