use eframe::egui;

use crate::{
    card::{CARD_SIZE, Card},
    theme::{
        colors, fonts,
        sizes::{self, ZONE_MARGIN_PX},
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CardOrientation {
    Portrait,
    Landscape,
}

impl CardOrientation {
    fn card_size(self) -> egui::Vec2 {
        match self {
            CardOrientation::Portrait => CARD_SIZE,
            CardOrientation::Landscape => egui::vec2(CARD_SIZE.y, CARD_SIZE.x),
        }
    }

    fn zone_size(self) -> egui::Vec2 {
        self.card_size() + egui::vec2(ZONE_MARGIN_PX, ZONE_MARGIN_PX)
    }
}

pub(super) struct Zone {
    title: String,
    orientation: CardOrientation,
    card: Option<Card>,
}

impl Zone {
    pub(super) fn new(title: impl Into<String>, orientation: CardOrientation) -> Self {
        Self {
            title: title.into(),
            orientation,
            card: None,
        }
    }

    pub(super) fn size(&self) -> egui::Vec2 {
        self.orientation.zone_size()
    }

    #[allow(dead_code)]
    pub(super) fn place_card(&mut self, card: Card) {
        self.card = Some(card);
    }

    pub(super) fn draw(&self, ui: &mut egui::Ui, center: egui::Pos2) {
        let rect = egui::Rect::from_center_size(center, self.size());

        ui.painter().rect_filled(rect, 2, colors::ZONE_FILL);

        ui.painter().rect_stroke(
            rect,
            sizes::ZONE_CORNER_RADIUS_PX,
            egui::Stroke::new(sizes::ZONE_STROKE_WIDTH_PX, colors::ZONE_STROKE),
            egui::StrokeKind::Inside,
        );

        match &self.card {
            Some(card) => {
                let card_rect = egui::Rect::from_center_size(center, self.orientation.card_size());
                card.draw(ui, card_rect);
            }
            None => {
                ui.painter().text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    &self.title,
                    egui::FontId::proportional(fonts::ZONE_TITLE_SIZE),
                    colors::ZONE_TEXT,
                );
            }
        }
    }
}
