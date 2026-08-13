use eframe::egui::{Pos2, Vec2};

use crate::domain::card::Card;
use crate::domain::zone::ZoneId;

/// Where a card being dragged came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardOrigin {
    TraySlot(usize),
    Zone(ZoneId),
}

pub struct DraggedCard {
    pub origin: CardOrigin,
    pub card: Card,
    pub grab_offset: Vec2,
    pub size: Vec2,
}

pub struct DropEvent {
    pub origin: CardOrigin,
    pub card: Card,
    pub drop_pos: Pos2,
}
