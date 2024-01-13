use crate::models::{card::Card, library::Library};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct LibraryResponse {
    id: Uuid,
    name: String,
    cards: Vec<(Uuid, Card)>,
}

impl LibraryResponse {
    pub fn from_library(library: Library, id: Uuid) -> Self {
        LibraryResponse {
            id,
            name: library.name,
            cards: library.cards.into_iter().collect(),
        }
    }
}
