use std::collections::HashMap;

use super::card::Card;
use super::zone::ZoneId;

#[derive(Default)]
pub struct Board {
    placed: HashMap<ZoneId, Card>,
}

impl Board {
    pub fn card_at(&self, zone: ZoneId) -> Option<&Card> {
        self.placed.get(&zone)
    }

    pub fn place(&mut self, zone: ZoneId, card: Card) -> Option<Card> {
        self.placed.insert(zone, card)
    }

    pub fn move_card(&mut self, from: ZoneId, to: ZoneId) {
        if from == to {
            return;
        }
        if let Some(card) = self.placed.remove(&from) {
            self.placed.insert(to, card);
        }
    }

    /// Removes and returns whatever card sits on `zone` (used to discard it).
    pub fn remove(&mut self, zone: ZoneId) -> Option<Card> {
        self.placed.remove(&zone)
    }
}
