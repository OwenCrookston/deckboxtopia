use axum::{
    body::Body,
    routing::{get, post},
    Router,
};

mod create_deck;
mod draw;
mod update_deck;

use create_deck::create_deck;
use draw::draw;
use update_deck::update;

use crate::state::ApiState;

pub fn routes() -> Router<ApiState, Body> {
    Router::new()
        .route("/:deck_id/draw", get(draw))
        .route("/create", post(create_deck))
        .route("/:deck_id", post(update))
}
