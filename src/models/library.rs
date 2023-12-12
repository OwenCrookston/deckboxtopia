use crate::endpoints::library::create_library::CreateLibraryResponse;

use super::card::Card;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    name: String,
    cards: HashMap<Uuid, Card>,
}

impl Library {
    /// creates a library
    pub fn new<Cards: IntoIterator<Item = Card>>(name: String, cards: Cards) -> Self {
        Library {
            name,
            cards: cards
                .into_iter()
                .map(|card| (Uuid::new_v4(), card))
                .collect(),
        }
    }

    /// looks up a mapped card by id and returns it
    pub fn get_card(&self, card_id: &Uuid) -> Option<&Card> {
        self.cards.get(card_id)
    }

    /// adds card to the library
    pub fn store_cards<Cards>(&mut self, cards: Cards)
    where
        Cards: IntoIterator<Item = Card>,
    {
        self.cards
            .extend(cards.into_iter().map(|card| (Uuid::new_v4(), card)));
    }

    pub fn new_empty(name: String) -> Self {
        Self::new(name, [])
    }

    /// gets the libraries cards
    pub fn get_cards(&self) -> impl Iterator<Item = (&Uuid, &Card)> {
        self.cards.iter()
    }

    /// constructs the create library response
    pub fn into_create_library_response(self, id: Uuid) -> CreateLibraryResponse {
        CreateLibraryResponse {
            id,
            name: self.name,
            cards: self.cards.into_iter().collect(),
        }
    }

    /// returns name of library
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// renames library
    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

#[cfg(test)]
pub mod tests {
    use url::Url;

    use crate::models::card::Card;

    use super::Library;

    pub fn test_cards() -> [Card; 5] {
        [
            Card::new("cat", Url::parse("http://google.com").unwrap()),
            Card::new("bat", Url::parse("http://google.com").unwrap()),
            Card::new("sat", Url::parse("http://google.com").unwrap()),
            Card::new("fat", Url::parse("http://google.com").unwrap()),
            Card::new("pat", Url::parse("http://google.com").unwrap()),
        ]
    }

    #[test]
    fn can_cards_store_and_access() {
        let cards = test_cards();
        let mut library = Library::new_empty("test library".to_string());
        library.store_cards(cards);
        assert_eq!(library.cards.len(), 5)
    }
}
