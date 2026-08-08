use eframe::egui;

use crate::theme::assets::PLAYING_BACKGROUND;

use super::{App, AppState};

impl App {
    pub(super) fn play_game(&mut self, ui: &mut egui::Ui) {
        let screen = ui.max_rect();

        egui::Image::new(PLAYING_BACKGROUND)
            .fit_to_exact_size(screen.size())
            .paint_at(ui, ui.max_rect());

        self.board.draw(ui);

        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.state = AppState::MainMenu;

            return;
        }
    }
}
