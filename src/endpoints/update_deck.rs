use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use shuttle_persist::PersistInstance;
use uuid::Uuid;

use crate::models::{deck::Deck, library::Library};

/// Updates a deck by adding a set of cards to it
/// Endpoint: `POST /decks/:deckId`
/// Body:
/// ```ignore
/// [
///    "id"
/// ]
/// ```
pub async fn update(
    State(shuttle_persist): State<PersistInstance>,
    Path(deck_id): Path<Uuid>,
    Json(card_ids): Json<Vec<Uuid>>,
) -> Result<(), StatusCode> {
    let deck_id = deck_id.to_string();

    let mut current_deck = shuttle_persist
        .load::<Deck>(&deck_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // check that the ids to be inserted reflect cards in the library
    let related_library = shuttle_persist
        .load::<Library>(&current_deck.get_library_id().to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // if card_id in card_ids is not found in library error out otherwise continue
    card_ids
        .iter()
        .find(|card_id| related_library.get_card(&card_id).is_some())
        .ok_or_else(|| StatusCode::NOT_FOUND)?;

    current_deck.insert_cards(card_ids);

    shuttle_persist
        .save(&deck_id, current_deck)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
