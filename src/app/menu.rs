use eframe::egui;

use super::{App, AppState};

impl App {
    pub(super) fn show_main_menu(&mut self, ui: &mut egui::Ui) {
        let screen = ui.max_rect();
        let background = egui::include_image!("../../assets/bg_branded.webp");

        egui::Image::new(background)
            .fit_to_exact_size(screen.size())
            .paint_at(ui, ui.max_rect());

        let menu_rect = egui::Rect::from_min_size(
            screen.min,
            egui::vec2(screen.width() / 3.0, screen.height()),
        );

        ui.scope_builder(egui::UiBuilder::new().max_rect(menu_rect), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(screen.height() * 0.25);

                let button_size = egui::vec2(menu_rect.width() * 0.8, 60.0);

                if ui
                    .add_sized(button_size, egui::Button::new("Nouvelle partie"))
                    .clicked()
                {
                    self.state = AppState::Playing;
                }
                ui.add_space(16.0);

                if ui
                    .add_sized(button_size, egui::Button::new("Continuer la partie"))
                    .clicked()
                {
                    // TODO
                }
                ui.add_space(16.0);

                if ui
                    .add_sized(button_size, egui::Button::new("Réglages"))
                    .clicked()
                {
                    // TODO
                }
                ui.add_space(16.0);

                if ui
                    .add_sized(button_size, egui::Button::new("Quitter le jeu"))
                    .clicked()
                {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}
