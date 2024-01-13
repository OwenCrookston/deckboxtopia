use axum::{
    extract::{Query, State},
    response::Redirect,
};
use serde::Deserialize;
use tracing::{debug, error, warn};

use crate::{
    auth::session::{Session, SessionData},
    state::ApiState,
};

#[derive(Deserialize)]
pub struct DiscordQuery {
    code: String,
    state: Option<String>,
}

pub async fn discord_callback(
    Query(query): Query<DiscordQuery>,
    State(state): State<ApiState>,
) -> Result<Redirect, Redirect> {
    debug!("exchanging code");

    let token_res = state
        .discord_api
        .exchange_code(query.code)
        .await
        .map_err(|e| {
            warn!(message = "unable to exchange code", err = ?e);
            Redirect::to(state.config.err_redirect_uri.as_str())
        })?;

    debug!("retrieving user info");

    let auth_info = state
        .discord_api
        .get_me(&token_res.access_token)
        .await
        .map_err(|e| {
            warn!(message = "unable to get user info", err = ?e);
            Redirect::to(state.config.err_redirect_uri.as_str())
        })?;

    debug!("encrypting session data");

    let session_data = SessionData::new(token_res, auth_info.user);

    let (encrypted, nonce) = session_data.encrypt(&state.cipher).map_err(|e| {
        warn!(message = "unable to encrypt session data", err = ?e);
        Redirect::to(state.config.err_redirect_uri.as_str())
    })?;

    debug!(message = "saving key", key = encrypted);

    let session = Session::new(encrypted);

    state.save_session(&session, nonce).map_err(|e| {
        warn!(message = "error when saving session", err = ?e);
        Redirect::to(state.config.err_redirect_uri.as_str())
    })?;

    let access_token = session.encode().map_err(|e| {
        error!(message = "error encoding session", err = ?e);
        Redirect::to(state.config.err_redirect_uri.as_str())
    })?;

    let params = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("access_token", &access_token)
        .finish();

    Ok(Redirect::to(&format!(
        "{}#{}",
        state.config.redirect_uri, params
    )))
}
