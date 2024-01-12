use self::sealed::Sealed;
use base64::{engine::general_purpose, Engine};
use surf::http::mime;

mod sealed {
    pub trait Sealed {}
}

pub trait RequestBuilderExt: Sealed {
    fn body_form(self, body: String) -> Self;
    fn auth_bearer(self, token: &str) -> Self;
    fn auth_basic(self, token: &str) -> Self;
    fn auth_client(self, client_id: &str, client_secret: &str) -> Self;
}

impl Sealed for surf::RequestBuilder {}

impl RequestBuilderExt for surf::RequestBuilder {
    fn body_form(self, body: String) -> Self {
        self.body_string(body).content_type(mime::FORM)
    }

    fn auth_bearer(self, token: &str) -> Self {
        self.header(
            surf::http::headers::AUTHORIZATION,
            format!("Bearer {}", token),
        )
    }

    fn auth_basic(self, token: &str) -> Self {
        self.header(
            surf::http::headers::AUTHORIZATION,
            format!("Basic {}", token),
        )
    }

    fn auth_client(self, client_id: &str, client_secret: &str) -> Self {
        self.auth_basic(
            &general_purpose::STANDARD.encode(format!("{}:{}", client_id, client_secret)),
        )
    }
}
