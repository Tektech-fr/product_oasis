use eframe::egui;

mod app;
mod board;
mod card;
mod theme;

use app::App;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };

    eframe::run_native(
        "L'Oasis",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::default()))
        }),
    )
}
