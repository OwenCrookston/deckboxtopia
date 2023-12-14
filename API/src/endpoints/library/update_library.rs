use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use shuttle_persist::PersistInstance;
use uuid::Uuid;

use crate::models::{card::Card, library::Library};

/// ```ignore
/// {
///     "id": "...",
///     "cards": [
///        {"name": "...", "url": "..."}
///     ]
/// }
/// ```
#[derive(Deserialize)]
pub struct UpdateLibraryRequest {
    name: Option<String>,
    cards: Vec<Card>,
}

/// Updates a library with new cards and potentially updates the library name
/// Endpoint: `POST /library/:library_id`
/// Body: CreateLibraryRequest
pub async fn update_library(
    State(shuttle_persist): State<PersistInstance>,
    Path(library_id): Path<Uuid>,
    Json(update_library_request): Json<UpdateLibraryRequest>,
) -> Result<(), StatusCode> {
    let library_id = library_id.to_string();

    // get the library from store
    let mut library_to_update = shuttle_persist
        .load::<Library>(&library_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // modify library
    library_to_update.store_cards(update_library_request.cards);
    if let Some(new_name) = update_library_request.name {
        library_to_update.update_name(new_name);
    }

    // resave modified library to store
    shuttle_persist
        .save(&library_id, library_to_update)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
