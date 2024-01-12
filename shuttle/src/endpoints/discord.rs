use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
};
use serde::Deserialize;
use tracing::{info, warn};

use crate::state::ApiState;

#[derive(Deserialize)]
pub struct DiscordQuery {
    code: String,
    state: Option<String>,
}

pub async fn discord_callback(
    Query(query): Query<DiscordQuery>,
    State(state): State<ApiState>,
) -> Result<Redirect, StatusCode> {
    //TODO: Error state should just redirect to crash page
    let res = state
        .discord_api
        .exchange_code(query.code)
        .await
        .map_err(|e| {
            warn!(message = "unable to exchange code", err = ?e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("retrieving user info");

    let auth_info = state
        .discord_api
        .get_me(&res.access_token)
        .await
        .map_err(|e| {
            warn!(message = "unable to get user info", err = ?e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(message = "auth_info", auth_info = ?auth_info);

    let params = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("access_token", &res.access_token)
        .finish();
    let redirect = Redirect::to(&format!("{}#{}", state.config.redirect_uri, params));

    Ok(redirect)
}
