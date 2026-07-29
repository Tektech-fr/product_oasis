use eframe::egui;

pub(super) struct Zone {
    title: String,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

impl Zone {
    pub(super) fn new(
        title: impl Into<String>,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            title: title.into(),
            left: left as f32 * 0.01,
            top: top as f32 * 0.01,
            width: width as f32 * 0.01,
            height: height as f32 * 0.01,
        }
    }

    fn rect_within(&self, board_rect: egui::Rect) -> egui::Rect {
        let min = board_rect.min
            + egui::vec2(
                self.left * board_rect.width(),
                self.top * board_rect.height(),
            );

        let size = egui::vec2(
            self.width * board_rect.width(),
            self.height * board_rect.height(),
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
