use super::card::Card;

#[derive(Default)]
pub struct Discard {
    cards: Vec<Card>,
}

impl Discard {
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}
