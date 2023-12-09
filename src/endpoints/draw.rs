use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use uuid::Uuid;

use crate::models::{
    card::Card,
    deck::{self, Deck},
    library::Library,
};

#[derive(Deserialize)]
#[serde(default)]
pub struct DrawQueryParams {
    number: usize,
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

#[derive(Serialize)]
pub struct DrawResponse {
    cards: Vec<(Uuid, Card)>,
    remaining_count: usize,
}

/// Draws a given number of cards from a given deck. Draw can be with or without replacement.
/// Endpoint: `GET /decks/:deckId/draw?number=1&replace=false`
/// Response:
/// ```
/// {
///     "cards": []
///     "remaining_count" : 5,
/// }
/// ```
pub async fn draw(
    State(shuttle_persist): State<PersistInstance>,
    Path(deck_id): Path<Uuid>,
    Query(params): Query<DrawQueryParams>,
) -> Result<Json<DrawResponse>, StatusCode> {
    let deck_id = deck_id.to_string();
    let mut current_deck = shuttle_persist
        .load::<Deck>(&deck_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let related_library = shuttle_persist
        .load::<Library>(&current_deck.get_library_id().to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (card_ids, remaining_count) = current_deck.draw_random(params.number);

    let response = DrawResponse {
        cards: card_ids
            .into_iter()
            .map(|card_id| {
                Ok((
                    card_id,
                    related_library
                        .get_card(&card_id)
                        .cloned()
                        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
                ))
            })
            .collect::<Result<_, StatusCode>>()?,
        remaining_count,
    };

    // save remaining undrawn cards as the updated deck
    shuttle_persist
        .save(&deck_id, current_deck)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(response))
}
