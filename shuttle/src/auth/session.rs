use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    AeadCore, Aes256Gcm,
};
use anyhow::anyhow;
use base64::{engine, Engine};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::discord;

use super::TokenResponse;

/// This contains the session data associated with the stored token.
/// When returned to the user this is the encrypted value that the user won't have access too
/// unless they have the key and nonce combination.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SessionData {
    pub token_res: TokenResponse,
    pub discord_user: discord::models::User,
    //TODO: Add Expiring date
}

impl SessionData {
    pub fn new(token_res: TokenResponse, discord_user: discord::models::User) -> Self {
        SessionData {
            token_res,
            discord_user,
        }
    }

    /// Encrypts the SessionData into a String an a Nonce used to encrypt
    ///
    /// One should limit the amount of nonces per key. Up to [4,294,967,296](https://docs.rs/aes-gcm/latest/aes_gcm/trait.AeadCore.html#method.generate_nonce) nonces per key. This should be enough space for this project since sessions should be removed fairly frequently.
    pub fn encrypt(&self, cipher: &Aes256Gcm) -> anyhow::Result<(String, Vec<u8>)> {
        let session_text = serde_json::to_string(self)?;
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let token = cipher
            .encrypt(&nonce, session_text.as_bytes())
            .map_err(|e| anyhow!(e.to_string()))?;

        let token = engine::general_purpose::URL_SAFE.encode(token);

        Ok((token, nonce.to_vec()))
    }

    pub fn decrypt(token: &str, nonce: Vec<u8>, cipher: &Aes256Gcm) -> anyhow::Result<Self> {
        let nonce = GenericArray::from_exact_iter(nonce.into_iter())
            .ok_or(anyhow!("nonce incorrect length"))?;

        let token = engine::general_purpose::URL_SAFE.decode(token)?;
        let token = cipher
            .decrypt(&nonce, token.as_ref())
            .map_err(|e| anyhow!(e.to_string()))?;
        let session_data = serde_json::from_slice(&token)?;

        Ok(session_data)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Session {
    token: String,
    kid: Uuid,
}

impl Session {
    pub fn new(token: String) -> Self {
        Session {
            token,
            kid: Uuid::new_v4(),
        }
    }

    pub fn get_kid(&self) -> &Uuid {
        &self.kid
    }

    pub fn encode(&self) -> anyhow::Result<String> {
        Ok(engine::general_purpose::URL_SAFE.encode(serde_json::to_vec(self)?))
    }

    pub fn decode(sess: &str) -> anyhow::Result<String> {
        Ok(serde_json::from_slice(
            &engine::general_purpose::URL_SAFE.decode(sess)?,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};

    use crate::{
        auth::{session::SessionData, TokenResponse},
        discord,
    };

    #[test]
    pub fn test_encrypt_decrypt() {
        let key = Aes256Gcm::generate_key(OsRng);
        let cipher = Aes256Gcm::new(&key);
        let session_data = SessionData {
            token_res: TokenResponse {
                access_token: "testToken".to_string(),
                refresh_token: "testRefreshToken".to_string(),
                token_type: "bearer".to_string(),
                expires_in: 3456,
                scope: "identity".to_string(),
            },
            discord_user: discord::models::User {
                id: 132455.into(),
                username: "IpFruion".to_string(),
                avatar: Some("13214".to_string()),
                global_name: Some("rust_bolt".to_string()),
            },
        };

        let (encoded, nonce) = session_data.encrypt(&cipher).expect("to be valid encode");
        println!("{}", encoded);

        let decoded = SessionData::decrypt(&encoded, nonce, &cipher).expect("to be valid decode");

        assert_eq!(session_data, decoded)
    }
}
