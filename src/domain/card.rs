#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardKind {
    Character,
    Building,
    Season,
}

impl CardKind {
    pub fn is_placeable(self) -> bool {
        matches!(self, Self::Character | Self::Building)
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub kind: CardKind,
}
