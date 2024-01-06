use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Deck {
    name: String,
    library_id: Uuid,
    card_ids: Vec<Uuid>,
}

impl Deck {
    /// Draws a number of cards randomly from the deck
    /// returns the cards drawn and a count of cards remaining in the deck
    /// saves the remaining deck
    pub fn draw_random(&mut self, count: usize) -> (Vec<Uuid>, usize) {
        // Create small, cheap to initialize and fast RNG with a random seed.
        // The randomness is supplied by the operating system.
        let mut small_rng = SmallRng::from_entropy();
        let (drawn_len, remaining_count) = {
            let (drawn, remaining_deck) = self.card_ids.partial_shuffle(&mut small_rng, count);
            (drawn.len(), remaining_deck.len())
        };

        // drain extracts the drawn cards from the deck and leaves the deck with the unshuffled elements
        let cards = self.card_ids.drain(0..drawn_len).collect();

        (cards, remaining_count)
    }

    pub fn get_library_id(&self) -> &Uuid {
        &self.library_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn insert_cards(&mut self, card_ids: Vec<Uuid>) {
        self.card_ids.extend(card_ids);
    }

    pub fn get_cards(&self) -> impl Iterator<Item = &Uuid> {
        self.card_ids.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Deck;

    fn test_shuffle() {
        let mut new_deck = Deck::default();

        let (card_ids, rest_of_deck) = new_deck.draw_random(3);
        println!("{:?}, {:?}", card_ids, rest_of_deck);
    }
}
