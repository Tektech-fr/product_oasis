use eframe::egui;

use super::zones::Zone;

pub(super) struct Board {
    zones: Vec<Zone>,
}

impl Board {
    pub(super) fn new() -> Self {
        Self {
            zones: vec![
                Zone::new("Réserve", egui::vec2(0.05, 0.05), egui::vec2(0.25, 0.20)),
                Zone::new("Pioche", egui::vec2(0.375, 0.05), egui::vec2(0.25, 0.20)),
                Zone::new("Défausse", egui::vec2(0.70, 0.05), egui::vec2(0.25, 0.20)),
                Zone::new(
                    "Main du joueur",
                    egui::vec2(0.05, 0.75),
                    egui::vec2(0.90, 0.20),
                ),
            ],
        }
    }

    pub(super) fn draw(&self, ui: &mut egui::Ui) {
        let window = ui.max_rect();
        let board_size = window.size() * 0.8;
        let board_rect = egui::Rect::from_center_size(window.center(), board_size);

        ui.painter().rect_stroke(
            board_rect,
            0,
            egui::Stroke::new(3.0, egui::Color32::WHITE),
            egui::StrokeKind::Inside,
        );

        for zone in &self.zones {
            zone.draw(ui, board_rect);
        }
    }
}
