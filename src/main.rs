use eframe::egui;

struct GameApp;


impl eframe::App for GameApp {
    fn ui(
        &mut self, 
        ui: &mut egui::Ui, 
        _frame: &mut eframe::Frame
    ) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
            egui::Image::new(egui::include_image!("../assets/bg_main_green_grass.webp"))
            .fit_to_exact_size(ui.max_rect().size())
            .paint_at(ui, ui.max_rect());

        ui.label("Bonjour");
        });
        }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_fullscreen(true),
        ..Default::default()
    };

    eframe::run_native(
        "L'Oasis",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
         Ok(Box::new(GameApp))
    }),
)
}