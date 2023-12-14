use axum::Router;
use shuttle_persist::PersistInstance;

mod deck;
pub mod library;

pub fn routes(shuttle_persist: PersistInstance) -> Router {
    let deck_routes = deck::routes();
    let library_routes = library::routes();
    Router::new()
        .nest("/deck", deck_routes)
        .nest("/library", library_routes)
        .with_state(shuttle_persist)
}
