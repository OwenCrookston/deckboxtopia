use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use uuid::Uuid;

use crate::models::{card::Card, library::Library};

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

#[derive(Serialize)]
pub struct CreateLibraryResponse {
    pub id: Uuid,
    pub name: String,
    pub cards: Vec<(Uuid, Card)>,
}

/// Creates a library by taking in a name and potentially cards
/// Endpoint: `POST /library`
/// Body: CreateLibraryRequest
pub async fn create_library(
    State(shuttle_persist): State<PersistInstance>,
    Json(create_library_request): Json<CreateLibraryRequest>,
) -> Result<Json<CreateLibraryResponse>, StatusCode> {
    // create a uuid for library
    let library_id = Uuid::new_v4();

    // create a library
    let new_library = Library::new(create_library_request.name, create_library_request.cards);

    // save library
    shuttle_persist
        .save(&library_id.to_string(), &new_library)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // build out CreateLibraryResponse and return it
    Ok(Json(new_library.into_create_library_response(library_id)))
}
