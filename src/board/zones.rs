use eframe::egui;

use crate::card::{CARD_SIZE, Card};

const ZONE_MARGIN: f32 = 10.0;

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
        self.card_size() + egui::vec2(ZONE_MARGIN, ZONE_MARGIN)
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

        ui.painter()
            .rect_filled(rect, 2, egui::Color32::from_black_alpha(80));

        ui.painter().rect_stroke(
            rect,
            2,
            egui::Stroke::new(1.5, egui::Color32::WHITE),
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
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );
            }
        }
    }
}
