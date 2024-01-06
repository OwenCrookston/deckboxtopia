use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    models::{deck::Deck, library::Library},
    state::ApiState,
};

/// Updates a deck by adding a set of cards to it
/// Endpoint: `POST /decks/:deckId`
/// Body:
/// ```ignore
/// [
///    "id"
/// ]
/// ```
pub async fn update(
    State(state): State<ApiState>,
    Path(deck_id): Path<Uuid>,
    Json(card_ids): Json<Vec<Uuid>>,
) -> Result<(), StatusCode> {
    let deck_id = deck_id.to_string();

    // load deck from store
    let mut current_deck = state
        .persist
        .load::<Deck>(&deck_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // check that the ids to be inserted reflect cards in the library
    let related_library = state
        .persist
        .load::<Library>(&current_deck.get_library_id().to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // if card_id in card_ids is not found in library error out otherwise continue
    card_ids
        .iter()
        .find(|card_id| related_library.get_card(&card_id).is_some())
        .ok_or_else(|| StatusCode::NOT_FOUND)?;

    current_deck.insert_cards(card_ids);

    state
        .persist
        .save(&deck_id, current_deck)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
