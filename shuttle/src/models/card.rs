use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    name: String,
    art_url: Url,
}

impl Card {
    /// contructs a card given a name and art_url
    pub fn new(name: &str, art_url: Url) -> Self {
        Card {
            name: name.to_string(),
            art_url,
        }
    }
}
