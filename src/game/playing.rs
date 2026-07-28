use eframe::egui;

use super::game_app::{AppState, GameApp};

impl GameApp {
    pub(super) fn play_game(&mut self, ui: &mut egui::Ui) {
        let screen = ui.max_rect();
        let background = egui::include_image!("../../assets/bg_green_grass.webp");

        egui::Image::new(background)
            .fit_to_exact_size(screen.size())
            .paint_at(ui, ui.max_rect());

        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.state = AppState::MainMenu;

            return;
        }
    }
}
