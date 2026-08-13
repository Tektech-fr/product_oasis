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
                name: "Toto".into(),
                description: "Membre de l'Oasis".into(),
                kind: CardKind::Character,
            },
            Card {
                id: 2,
                name: "Cabane".into(),
                description: "Un endroit pour se loger".into(),
                kind: CardKind::Building,
            },
            Card {
                id: 3,
                name: "Saison : Été".into(),
                description: "La canicule, y a que ça de vrai".into(),
                kind: CardKind::Season,
            },
        ];
        Self {
            board: Board::default(),
            tray: Tray::new(starter),
            discard: Discard::default(),
        }
    }

    /// Places a tray card on `zone`.
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
