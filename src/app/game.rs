use crate::domain::board::Board;
use crate::domain::card::{Card, CardKind};
use crate::domain::discard::Discard;
use crate::domain::tray::Tray;
use crate::domain::zone::ZoneId;

pub struct Game {
    pub board: Board,
    pub tray: Tray,
    pub discard: Discard,
}

impl Game {
    pub fn new() -> Self {
        let starter = [
            Card {
                id: 1,
                name: "Membre de l'Oasis".into(),
                description: String::new(),
                image_path: String::new(),
                kind: CardKind::Character,
            },
            Card {
                id: 2,
                name: "Cabane".into(),
                description: String::new(),
                image_path: String::new(),
                kind: CardKind::Building,
            },
            Card {
                id: 3,
                name: "Saison : Été".into(),
                description: String::new(),
                image_path: String::new(),
                kind: CardKind::Season,
            },
        ];
        Self {
            board: Board::default(),
            tray: Tray::new(starter),
            discard: Discard::default(),
        }
    }

    /// Places a tray card on `zone`. No-op if the card isn't a placeable kind.
    pub fn place_from_tray(&mut self, slot: usize, card: Card, zone: ZoneId) {
        if card.kind.is_placeable() {
            self.board.place(zone, card);
            self.tray.clear(slot);
        }
    }

    /// Moves a card already on the board from one zone to another.
    pub fn move_card(&mut self, from: ZoneId, to: ZoneId) {
        self.board.move_card(from, to);
    }

    /// Takes a card off the board and sends it to the discard pile.
    pub fn discard_card(&mut self, zone: ZoneId) {
        if let Some(card) = self.board.remove(zone) {
            self.discard.add(card);
        }
    }
}
