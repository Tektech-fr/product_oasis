use eframe::egui::{Image, ImageSource, Ui};

pub fn paint_fullscreen(ui: &mut Ui, image: ImageSource<'static>) {
    let screen = ui.max_rect();

    Image::new(image)
        .fit_to_exact_size(screen.size())
        .paint_at(ui, screen);
}
