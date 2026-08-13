use eframe::egui::{self, Button, Image, ImageSource, Ui, vec2};
use egui_extras::{Size, StripBuilder};

#[derive(Clone, Copy)]
pub enum MenuAction {
    NewGame,
    Quit,
}

const BACKGROUND: ImageSource<'static> = egui::include_image!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/shared/bg_yellow_board.webp"
));
const TITLE: ImageSource<'static> =
    egui::include_image!(concat!(env!("CARGO_MANIFEST_DIR"), "/shared/bg_title.webp"));

const MENU_ITEMS: [(&str, MenuAction); 2] = [
    ("Nouvelle partie", MenuAction::NewGame),
    ("Quitter", MenuAction::Quit),
];

const PANEL_WIDTH_RATIO: f32 = 0.5;
const TITLE_HEIGHT_RATIO: f32 = 0.25;
const BUTTON_WIDTH_RATIO: f32 = 0.6;
const BUTTON_HEIGHT: f32 = 70.0;
const BUTTON_GAP: f32 = 40.0;

pub fn show(ui: &mut Ui) -> Option<MenuAction> {
    let screen = ui.max_rect();
    Image::new(BACKGROUND)
        .fit_to_exact_size(screen.size())
        .paint_at(ui, screen);

    let mut action = None;

    StripBuilder::new(ui)
        .size(Size::relative(PANEL_WIDTH_RATIO))
        .size(Size::remainder())
        .horizontal(|mut strip| {
            strip.strip(|builder| {
                builder
                    .size(Size::relative(TITLE_HEIGHT_RATIO))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.cell(show_title);
                        strip.cell(|ui| action = show_buttons(ui));
                    });
            });
            strip.empty();
        });

    action
}

fn show_title(ui: &mut Ui) {
    let area = ui.max_rect();
    Image::new(TITLE)
        .fit_to_exact_size(area.size())
        .paint_at(ui, area);
}

fn show_buttons(ui: &mut Ui) -> Option<MenuAction> {
    let mut action = None;
    let button_size = vec2(ui.available_width() * BUTTON_WIDTH_RATIO, BUTTON_HEIGHT);

    let count = MENU_ITEMS.len() as f32;
    let content_height = count * BUTTON_HEIGHT + (count - 1.0) * BUTTON_GAP;
    let top_margin = (ui.available_height() - content_height).max(0.0) / 2.0;

    ui.vertical_centered(|ui| {
        ui.add_space(top_margin);
        for (i, (label, item)) in MENU_ITEMS.iter().enumerate() {
            if i > 0 {
                ui.add_space(BUTTON_GAP);
            }
            if ui.add_sized(button_size, Button::new(*label)).clicked() {
                action = Some(*item);
            }
        }
    });

    action
}
