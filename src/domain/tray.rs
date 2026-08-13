use super::card::Card;

const SLOT_COUNT: usize = 3;

pub struct Tray {
    slots: [Option<Card>; SLOT_COUNT],
}

impl Tray {
    pub fn new(starter: [Card; SLOT_COUNT]) -> Self {
        Self {
            slots: starter.map(Some),
        }
    }

    pub fn slots(&self) -> impl Iterator<Item = (usize, &Card)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, slot)| slot.as_ref().map(|c| (i, c)))
    }

    pub fn clear(&mut self, index: usize) {
        self.slots[index] = None;
    }
}
