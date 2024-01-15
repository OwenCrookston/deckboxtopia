use crate::{
    auth::{AuthorizationCode, GrantType, TokenResponse},
    utils::RequestBuilderExt,
};
use anyhow::anyhow;
use tracing::warn;

use self::models::AuthorizationInfo;

pub mod config;
pub mod models;
pub use config::Config;

pub struct DiscordApi {
    client: surf::Client,
    config: Config,
}

impl DiscordApi {
    pub fn new(client: surf::Client, config: Config) -> Self {
        DiscordApi { client, config }
    }

    pub async fn exchange_code(&self, code: String) -> anyhow::Result<TokenResponse> {
        let grant_type = GrantType::AuthorizationCode(AuthorizationCode::new(
            code,
            self.config.redirect_uri.to_string(),
        ));
        self.send_token(grant_type).await
    }

    pub async fn send_token(&self, grant_type: GrantType) -> anyhow::Result<TokenResponse> {
        let uri = format!("{}/oauth2/token", self.config.api_endpoint);
        let req = grant_type
            .add_body(surf::post(uri))
            .auth_client(&self.config.client_id, &self.config.client_secret);

        let mut res = self
            .client
            .send(req)
            .await
            .map_err(|e| anyhow!(e.to_string()))?;

        if !res.status().is_success() {
            let body = res
                .body_string()
                .await
                .map_err(|e| anyhow!(e.to_string()))?;
            warn!(message = "response code not success", body = %body, status = ?res.status());

            return Err(anyhow!("error requesting token"));
        }

        res.body_json().await.map_err(|e| anyhow!(e.to_string()))
    }

    pub async fn get_me(&self, token: &str) -> anyhow::Result<AuthorizationInfo> {
        let uri = format!("{}/oauth2/@me", self.config.api_endpoint);
        let req = surf::get(uri).auth_bearer(token);

        let mut res = self
            .client
            .send(req)
            .await
            .map_err(|_| anyhow!("something went wrong"))?;

        if !res.status().is_success() {
            let body = res
                .body_string()
                .await
                .map_err(|e| anyhow!(e.to_string()))?;
            warn!(message = "couldn't retrieve @me", body = %body, status = ?res.status());

            return Err(anyhow!("error requesting get me"));
        }
        res.body_json().await.map_err(|e| anyhow!(e.to_string()))
    }
}
