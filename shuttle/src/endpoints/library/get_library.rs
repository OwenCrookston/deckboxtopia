use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use super::models::LibraryResponse;
use crate::{models::library::Library, state::ApiState};
use tracing::warn;

pub async fn get_library(
    State(state): State<ApiState>,
    Path(library_id): Path<Uuid>,
) -> Result<Json<LibraryResponse>, StatusCode> {
    // get the library from store
    let library = state
        .persist
        .load::<Library>(&library_id.to_string())
        .map_err(|err| {
            warn!(message = "invalid library", err = ?err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LibraryResponse::from_library(library, library_id)))
}
