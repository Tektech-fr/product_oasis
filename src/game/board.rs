use eframe::egui;

use super::zones::Zone;

pub(super) struct Board {
    zones: Vec<Zone>,
}

impl Board {
    pub(super) fn new() -> Self {
        Self {
            zones: vec![
                Zone::new("Aménagement A", 5, 5, 80, 80),
                Zone::new("Aménagement B", 5, 5, 80, 80),
                Zone::new("Aménagement C", 5, 5, 80, 80),
                Zone::new("Aménagement D", 5, 5, 80, 80),
                Zone::new("Aménagement E", 5, 5, 80, 80),
                Zone::new("Contruction", 5, 5, 80, 80),
                Zone::new("Marché", 5, 5, 80, 80),
                Zone::new("Réserve", 20, 20, 70, 70),
                Zone::new("Cabane", 30, 30, 60, 60),
                Zone::new("Tente A", 50, 50, 50, 50),
                Zone::new("Tente B", 5, 5, 80, 80),
                Zone::new("Résidence d'artiste", 5, 5, 80, 80),
                Zone::new("Cuisine", 5, 5, 80, 80),
                Zone::new("Hygiène et Intendance", 5, 5, 80, 80),
                Zone::new("Centre du Foyer", 5, 5, 80, 80),
                Zone::new("Bois A", 5, 5, 80, 80),
                Zone::new("Bois B", 5, 5, 80, 80),
                Zone::new("Champs A", 5, 5, 80, 80),
                Zone::new("Champs B", 5, 5, 80, 80),
                Zone::new("Champs C", 5, 5, 80, 80),
            ],
        }
    }

    pub(super) fn draw(&self, ui: &mut egui::Ui) {
        let window = ui.max_rect();
        let board_size = window.size() * 0.9;
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
