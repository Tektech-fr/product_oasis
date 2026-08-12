use eframe::egui::Color32;

use crate::domain::card::CardKind;
use crate::domain::zone::ZoneId;

pub mod layout {
    pub const CARD_SIZE_MM: (f32, f32) = (85.0, 55.0);
    pub const CARD_ASPECT: f32 = CARD_SIZE_MM.0 / CARD_SIZE_MM.1;
    // Board & layout related constants used by UI layout code
    pub const SIDE_WIDTH_FRACTION: f32 = 0.16;
    pub const BOARD_MARGIN: f32 = 24.0;
    pub const DISCARD_HEIGHT_FRACTION: f32 = 0.22;

    // Spacing inside UI grid and card margins
    pub const CELL_GAP: f32 = 6.0;
    pub const CARD_MARGIN_IN_CELL: f32 = 6.0;
}

pub mod colors {
    use eframe::egui::Color32;

    pub const STROKES: Color32 = Color32::WHITE;
}

pub fn zone_color(zone: ZoneId) -> Color32 {
    match zone {
        ZoneId::Amenagement(_) => Color32::from_rgb(220, 230, 167),
        ZoneId::ResidenceArtiste
        | ZoneId::Cuisine
        | ZoneId::HygieneEtIntendance
        | ZoneId::FoyerCentral => Color32::from_rgb(236, 157, 118),
        ZoneId::Construction => Color32::from_rgb(231, 47, 169),
        ZoneId::Reserve => Color32::from_rgb(35, 40, 166),
        ZoneId::Cabane => Color32::from_rgb(168, 83, 76),
        ZoneId::Tente(_) => Color32::from_rgb(84, 215, 109),
        ZoneId::Marche => Color32::from_rgb(141, 119, 33),
        ZoneId::Bois(_) => Color32::from_rgb(20, 98, 22),
        ZoneId::Champ(_) => Color32::from_rgb(74, 28, 28),
    }
}

pub fn card_color(kind: CardKind) -> Color32 {
    match kind {
        CardKind::Character => Color32::from_rgb(240, 200, 90),
        CardKind::Building => Color32::from_rgb(180, 110, 70),
        CardKind::Season => Color32::from_rgb(120, 150, 200),
    }
}

pub fn readable_text_color(bg: Color32) -> Color32 {
    let luminance = 0.299 * bg.r() as f32 + 0.587 * bg.g() as f32 + 0.114 * bg.b() as f32;
    if luminance > 140.0 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}
