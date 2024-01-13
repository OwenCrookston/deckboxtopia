use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use uuid::Uuid;

use super::models::LibraryResponse;
use crate::{
    models::{card::Card, library::Library},
    state::ApiState,
};

/// ```ignore
/// {
///     "name": "...",
///     "cards": [
///        {"name": "...", "url": "..."}
///     ]
/// }
/// ```
#[derive(Deserialize)]
pub struct CreateLibraryRequest {
    name: String,
    cards: Vec<Card>,
}

/// Creates a library by taking in a name and potentially cards
/// Endpoint: `POST /library`
/// Body: CreateLibraryRequest
pub async fn create_library(
    State(state): State<ApiState>,
    Json(create_library_request): Json<CreateLibraryRequest>,
) -> Result<Json<LibraryResponse>, StatusCode> {
    // create a uuid for library
    let library_id = Uuid::new_v4();

    // create a library
    let new_library = Library::new(create_library_request.name, create_library_request.cards);

    // save library
    state
        .persist
        .save(&library_id.to_string(), &new_library)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // build out CreateLibraryResponse and return it
    Ok(Json(LibraryResponse::from_library(new_library, library_id)))
}
