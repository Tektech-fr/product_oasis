use eframe::egui::{self, ImageSource, Ui};

use crate::ui::views::background_view::paint_fullscreen;

const BACKGROUND_GAME: ImageSource<'static> = egui::include_image!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/shared/bg_green_grass.webp"
));

pub fn show(ui: &mut Ui) {
    paint_fullscreen(ui, BACKGROUND_GAME);
}
