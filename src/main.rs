mod app;
mod domain;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_title("L'Oasis des Tournesols"),
        ..Default::default()
    };

    eframe::run_native(
        "L'Oasis des Tournesols",
        options,
        Box::new(|cc| Ok(Box::new(ui::screens::app_screen::OasisApp::new(cc)))),
    )
}

