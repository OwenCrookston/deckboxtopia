use axum::{
    body::Body,
    routing::{get, post},
    Router,
};

pub mod create_library;
mod get_library;
mod models;
mod update_library;

use create_library::create_library;
use get_library::get_library;
use update_library::update_library;

use crate::state::ApiState;

pub fn routes() -> Router<ApiState, Body> {
    Router::new()
        .route("/create", post(create_library))
        .route(":library_id", get(get_library))
        .route("/update/:library_id", post(update_library))
}
