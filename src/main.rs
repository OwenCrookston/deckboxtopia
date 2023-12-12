mod endpoints;
mod models;
use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use endpoints::{
    deck::create_deck::create_deck, deck::draw::draw, deck::update_deck::update,
    library::create_library::create_library,
};
use shuttle_persist::PersistInstance;
use tower_http::services::ServeDir;
use tracing::warn;

#[shuttle_runtime::main]
async fn main(#[shuttle_persist::Persist] persist: PersistInstance) -> shuttle_axum::ShuttleAxum {
    if let Err(err) = persist.clear() {
        warn!("Error clearing persistance: {:?}", err);
    }

    let deck_routes = Router::new()
        .route("/:deck_id/draw", get(draw))
        .route("/create", post(create_deck))
        .route("/:deck_id", post(update));

    let library_routes = Router::new().route("/create", post(create_library));

    let router = Router::new()
        .nest_service("/ui", ServeDir::new("static"))
        .route("/", get(|| async { Redirect::to("/ui") }))
        .nest("/deck", deck_routes)
        .nest("/library", library_routes)
        .with_state(persist);

    Ok(router.into())
}
