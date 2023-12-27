use axum::{body::Body, routing::post, Router};

pub mod create_library;
mod update_library;

use create_library::create_library;
use update_library::update_library;

use crate::state::ApiState;

pub fn routes() -> Router<ApiState, Body> {
    Router::new()
        .route("/create", post(create_library))
        .route("/update/:library_id", post(update_library))
}
