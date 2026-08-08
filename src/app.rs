use eframe::egui;

use crate::board::Board;

mod menu;
mod playing;

#[derive(PartialEq)]
pub(crate) enum AppState {
    MainMenu,
    Playing,
}

pub struct App {
    pub(crate) state: AppState,
    pub(crate) board: Board,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            board: Board::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| match self.state {
                AppState::MainMenu => self.show_main_menu(ui),
                AppState::Playing => self.play_game(ui),
            });
    }
}
