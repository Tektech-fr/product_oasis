use eframe::egui;

use crate::theme::{colors, fonts, sizes};

pub(crate) const CARD_SIZE: egui::Vec2 = egui::vec2(70.0, 100.0);

pub(crate) struct Card {
    name: String,
}

impl Card {
    pub(crate) fn draw(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        ui.painter()
            .rect_filled(rect, sizes::CARD_CORNER_RADIUS_PX, colors::CARD_FILL);

        ui.painter().rect_stroke(
            rect,
            sizes::CARD_CORNER_RADIUS_PX,
            egui::Stroke::new(sizes::CARD_STROKE_WIDTH_PX, colors::CARD_STROKE),
            egui::StrokeKind::Inside,
        );

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.name,
            egui::FontId::proportional(fonts::CARD_LABEL_SIZE),
            colors::CARD_TEXT,
        );
    }
}
