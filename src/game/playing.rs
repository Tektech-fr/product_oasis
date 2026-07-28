use eframe::egui;

use super::game_app::{AppState, GameApp};

impl GameApp {
    pub(super) fn play_game(&mut self, ui: &mut egui::Ui) {
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.state = AppState::MainMenu;

            return;
        }
    }
}
