use axum::{
    extract::{Path, Query},
    http::StatusCode,
};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct DrawQueryParams {
    number: u32,
    replace: bool,
}

impl Default for DrawQueryParams {
    fn default() -> Self {
        Self {
            number: 1,
            replace: false,
        }
    }
}

/// Draws a given number of cards from a given deck. Draw can be with or without replacement.
/// Endpoint: `GET /decks/:deckId/draw?number=1&replace=false`
pub async fn draw(Path(deck_id): Path<u32>, Query(params): Query<DrawQueryParams>) -> String {
    format!(
        "deck_id: {}\nnumber: {}\nreplace: {}",
        deck_id, params.number, params.replace
    )
}
