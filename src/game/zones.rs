use eframe::egui;

pub(super) struct Zone {
    title: String,
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
}

impl Zone {
    pub(super) fn new(
        title: impl Into<String>,
        center_x: usize,
        center_y: usize,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            title: title.into(),
            center_x: center_x as f32 * 0.01,
            center_y: center_y as f32 * 0.01,
            width: width as f32 * 0.01,
            height: height as f32 * 0.01,
        }
    }

    fn rect_within(&self, board_rect: egui::Rect) -> egui::Rect {
        let center = board_rect.min
            + egui::vec2(
                self.center_x * board_rect.width(),
                self.center_y * board_rect.height(),
            );

        let size = egui::vec2(
            self.width * board_rect.width(),
            self.height * board_rect.height(),
        );

        egui::Rect::from_center_size(center, size)
    }

    pub(super) fn draw(&self, ui: &mut egui::Ui, board_rect: egui::Rect) {
        let rect = self.rect_within(board_rect);

        ui.painter()
            .rect_filled(rect, 0, egui::Color32::from_black_alpha(80));

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
