use eframe::egui;

pub(super) struct Zone {
    title: String,
    relative_pos: egui::Vec2,
    relative_size: egui::Vec2,
}

impl Zone {
    pub(super) fn new(
        title: impl Into<String>,
        relative_pos: egui::Vec2,
        relative_size: egui::Vec2,
    ) -> Self {
        Self {
            title: title.into(),
            relative_pos,
            relative_size,
        }
    }

    fn rect_within(&self, board_rect: egui::Rect) -> egui::Rect {
        let min = board_rect.min
            + egui::vec2(
                self.relative_pos.x * board_rect.width(),
                self.relative_pos.y * board_rect.height(),
            );

        let size = egui::vec2(
            self.relative_size.x * board_rect.width(),
            self.relative_size.y * board_rect.height(),
        );

        egui::Rect::from_min_size(min, size)
    }

    pub(super) fn draw(&self, ui: &mut egui::Ui, board_rect: egui::Rect) {
        let rect = self.rect_within(board_rect);

        ui.painter()
            .rect_filled(rect, 0, egui::Color32::from_black_alpha(60));
        ui.painter().rect_stroke(
            rect,
            0,
            egui::Stroke::new(1.5, egui::Color32::WHITE),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &self.title,
            egui::FontId::proportional(16.0),
            egui::Color32::WHITE,
        );
    }
}
