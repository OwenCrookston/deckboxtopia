use axum::Router;

use crate::state::ApiState;

mod deck;
pub mod library;

pub fn routes(state: ApiState) -> Router {
    let deck_routes = deck::routes();
    let library_routes = library::routes();
    Router::new()
        .nest("/deck", deck_routes)
        .nest("/library", library_routes)
        .with_state(state)
}
