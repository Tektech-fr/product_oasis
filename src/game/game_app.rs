use eframe::egui;

#[derive(PartialEq)]
pub(super) enum AppState {
    MainMenu,
    Playing,
}
pub struct GameApp {
    pub(super) state: AppState,
}

impl Default for GameApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
        }
    }
}

impl eframe::App for GameApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| match self.state {
                AppState::MainMenu => self.show_main_menu(ui),
                AppState::Playing => self.play_game(ui),
            });
    }
}
