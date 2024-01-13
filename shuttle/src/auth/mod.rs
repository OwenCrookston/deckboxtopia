pub mod session;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::utils::RequestBuilderExt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    //TODO: Make duration?
    pub expires_in: u32,
    pub scope: String,
}

pub enum GrantType {
    AuthorizationCode(AuthorizationCode),
}

impl GrantType {
    pub fn add_body(&self, req_builder: surf::RequestBuilder) -> surf::RequestBuilder {
        match self {
            GrantType::AuthorizationCode(a) => a.add_body(req_builder),
        }
    }
}

pub struct AuthorizationCode {
    code: String,
    redirect_uri: String,
}

impl AuthorizationCode {
    pub fn new(code: String, redirect_uri: String) -> AuthorizationCode {
        AuthorizationCode { code, redirect_uri }
    }

    fn add_body(&self, req_builder: surf::RequestBuilder) -> surf::RequestBuilder {
        let body = self.to_form();
        info!(message = "auth code body", body = %body);
        req_builder.body_form(body)
    }

    fn to_form(&self) -> String {
        url::form_urlencoded::Serializer::new(String::new())
            .append_pair("grant_type", "authorization_code")
            .append_pair("code", &self.code)
            .append_pair("redirect_uri", &self.redirect_uri)
            .finish()
    }
}
