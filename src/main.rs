mod endpoints;
use axum::{response::Redirect, routing::get, Router};
use endpoints::draw::draw;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .nest_service("/ui", ServeDir::new("static"))
        .route("/", get(|| async { Redirect::to("/ui") }))
        .route("/decks/:deck_id/draw", get(draw));

    Ok(router.into())
}
