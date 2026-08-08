use eframe::egui;

use crate::theme::{assets, ratios, sizes, spacings};

use super::{App, AppState};

impl App {
    pub(super) fn show_main_menu(&mut self, ui: &mut egui::Ui) {
        let screen = ui.max_rect();

        egui::Image::new(assets::MENU_BACKGROUND)
            .fit_to_exact_size(screen.size())
            .paint_at(ui, ui.max_rect());

        let menu_rect = egui::Rect::from_min_size(
            screen.min,
            egui::vec2(
                screen.width() / sizes::MENU_WIDTH_DIVISOR,
                screen.height() / sizes::MENU_HEIGHT_DIVISOR,
            ),
        );

        ui.scope_builder(egui::UiBuilder::new().max_rect(menu_rect), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(screen.height() * ratios::MENU_TOP_SPACING);

                let button_size = egui::vec2(
                    menu_rect.width() * ratios::BUTTON_WIDTH,
                    menu_rect.height() * ratios::BUTTON_HEIGHT,
                );

                if ui
                    .add_sized(button_size, egui::Button::new("Nouvelle partie"))
                    .clicked()
                {
                    self.state = AppState::Playing;
                }
                ui.add_space(spacings::MD_PX);

                if ui
                    .add_sized(button_size, egui::Button::new("Continuer la partie"))
                    .clicked()
                {
                    // TODO
                }
                ui.add_space(spacings::MD_PX);

                if ui
                    .add_sized(button_size, egui::Button::new("Réglages"))
                    .clicked()
                {
                    // TODO
                }
                ui.add_space(spacings::MD_PX);

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
