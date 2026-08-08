use eframe::egui;

pub(crate) const CARD_SIZE: egui::Vec2 = egui::vec2(70.0, 100.0);

pub(crate) struct Card {
    name: String,
}

impl Card {
    pub(crate) fn draw(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        ui.painter()
            .rect_filled(rect, 4, egui::Color32::from_rgb(235, 225, 205));

        ui.painter().rect_stroke(
            rect,
            4,
            egui::Stroke::new(1.5, egui::Color32::BLACK),
            egui::StrokeKind::Inside,
        );

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.name,
            egui::FontId::proportional(14.0),
            egui::Color32::BLACK,
        );
    }
}
