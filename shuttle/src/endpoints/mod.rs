mod deck;
mod discord;
pub mod library;

use axum::{routing::get, Router};

use crate::state::ApiState;

use self::discord::discord_callback;

pub fn routes(state: ApiState) -> Router {
    let deck_routes = deck::routes();
    let library_routes = library::routes();
    Router::new()
        .nest("/deck", deck_routes)
        .nest("/library", library_routes)
        .route("/discord/callback", get(discord_callback))
        .with_state(state)
}
