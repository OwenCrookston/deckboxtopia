use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use shuttle_persist::PersistInstance;
use uuid::Uuid;

use crate::models::{card::Card, deck::Deck, library::Library};

#[derive(Serialize)]
pub struct CreateDeckResponse {
    library_name: String,
    deck_id: Uuid,
    cards: Vec<Card>,
}
/// Creates a deck by taking in a name, library_id, and a set of card ids
/// Guaranteed to get back the cards array in the same order as the card ids came in
/// Errors if a card cannot be found by id
/// Endpoint: `POST /decks`
/// Body:
/// ```ignore
/// {
///     "name": "...",
///     "library_id": "id"
///     "cards": [
///        "id"
///     ]
/// }
/// ```
pub async fn create_deck(
    State(shuttle_persist): State<PersistInstance>,
    Json(new_deck): Json<Deck>,
) -> Result<Json<CreateDeckResponse>, StatusCode> {
    let new_deck_id = Uuid::new_v4();

    let library = shuttle_persist
        .load::<Library>(&new_deck.get_library_id().to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    shuttle_persist
        .save(&new_deck_id.to_string(), &new_deck)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateDeckResponse {
        library_name: library.get_name().to_string(),
        deck_id: new_deck_id,
        cards: new_deck
            .get_cards()
            .map(|card_id| {
                library
                    .get_card(card_id)
                    .cloned()
                    .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
            })
            .collect::<Result<_, StatusCode>>()?,
    }))
}
