use eframe::egui::{CentralPanel, Ui};

pub enum MenuAction {
    NewGame,
    Quit,
}

pub fn show(ui: &mut Ui) -> Option<MenuAction> {
    let mut action = None;

    CentralPanel::default().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            ui.heading("L'Oasis des Tournesols");
            ui.add_space(24.0);
            if ui.button("Nouvelle partie").clicked() {
                action = Some(MenuAction::NewGame);
            }
            if ui.button("Quitter").clicked() {
                action = Some(MenuAction::Quit);
            }
        });
    });

    action
}
