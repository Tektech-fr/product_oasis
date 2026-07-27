use eframe::egui;

#[derive(Default)]
struct GameApp;


impl eframe::App for GameApp {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        ui.label("Bonjour");
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "L'Oasis",
        options,
        Box::new(|_cc| Ok(Box::new(GameApp))),
    )
}