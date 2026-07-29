use eframe::egui;

use super::zones::Zone;

pub(super) struct Board {
    zones: Vec<Zone>,
}

impl Board {
    pub(super) fn new() -> Self {
        Self {
            zones: vec![
                Zone::new("Aménagement A", 10, 10, 10, 15),
                Zone::new("Aménagement B", 30, 10, 10, 15),
                Zone::new("Aménagement C", 50, 10, 10, 15),
                Zone::new("Aménagement D", 70, 10, 10, 15),
                Zone::new("Aménagement E", 90, 10, 10, 15),
                Zone::new("Contruction", 10, 30, 15, 10),
                Zone::new("Marché", 10, 45, 15, 10),
                Zone::new("Réserve", 10, 60, 15, 15),
                Zone::new("Cabane", 35, 50, 10, 15),
                Zone::new("Tente A", 50, 50, 10, 15),
                Zone::new("Tente B", 65, 50, 10, 15),
                Zone::new("Résidence d'artiste", 90, 30, 15, 10),
                Zone::new("Cuisine", 90, 45, 15, 10),
                Zone::new("Hygiène et Intendance", 90, 60, 15, 10),
                Zone::new("Centre du Foyer", 90, 75, 15, 10),
                Zone::new("Bois A", 10, 90, 10, 15),
                Zone::new("Bois B", 30, 90, 10, 15),
                Zone::new("Champs A", 50, 90, 10, 15),
                Zone::new("Champs B", 70, 90, 10, 15),
                Zone::new("Champs C", 90, 90, 10, 15),
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
