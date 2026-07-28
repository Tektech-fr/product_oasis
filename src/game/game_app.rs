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
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                let background = match self.state {
                    AppState::MainMenu => egui::include_image!("../../assets/bg_branded.webp"),
                    AppState::Playing => {
                        egui::include_image!("../../assets/bg_green_grass.webp")
                    }
                };

                egui::Image::new(background)
                    .fit_to_exact_size(ui.max_rect().size())
                    .paint_at(ui, ui.max_rect());

                if self.state == AppState::MainMenu {
                    self.show_main_menu(ui);
                }
            });
    }
}
