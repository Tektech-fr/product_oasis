pub(crate) mod assets {
    use eframe::egui;

    pub(crate) const MENU_BACKGROUND: egui::ImageSource<'static> =
        egui::include_image!("../assets/bg_branded.webp");

    pub(crate) const PLAYING_BACKGROUND: egui::ImageSource<'static> =
        egui::include_image!("../assets/bg_green_grass.webp");
}

pub(crate) mod colors {
    use eframe::egui::Color32;

    pub(crate) const BOARD_STROKE: Color32 = Color32::WHITE;
    pub(crate) const ZONE_FILL: Color32 = Color32::from_black_alpha(80);
    pub(crate) const ZONE_STROKE: Color32 = Color32::WHITE;
    pub(crate) const ZONE_TEXT: Color32 = Color32::WHITE;
    pub(crate) const CARD_FILL: Color32 = Color32::from_rgb(235, 225, 205);
    pub(crate) const CARD_STROKE: Color32 = Color32::BLACK;
    pub(crate) const CARD_TEXT: Color32 = Color32::BLACK;
}

pub(crate) mod fonts {
    pub(crate) const ZONE_TITLE_SIZE: f32 = 14.0;
    pub(crate) const CARD_LABEL_SIZE: f32 = 14.0;
}

pub(crate) mod ratios {
    pub(crate) const MENU_TOP_SPACING: f32 = 0.25;
    pub(crate) const BUTTON_WIDTH: f32 = 0.8;
    pub(crate) const BUTTON_HEIGHT: f32 = 1.0 / 8.0;
    pub(crate) const BOARD_SIZE: f32 = 0.8;
}

pub(crate) mod sizes {
    pub(crate) const MENU_WIDTH_DIVISOR: f32 = 3.0;
    pub(crate) const MENU_HEIGHT_DIVISOR: f32 = 2.0;
    pub(crate) const ZONE_MARGIN_PX: f32 = 10.0;
    pub(crate) const BOARD_STROKE_WIDTH_PX: f32 = 3.0;
    pub(crate) const ZONE_STROKE_WIDTH_PX: f32 = 1.5;
    pub(crate) const ZONE_CORNER_RADIUS_PX: u8 = 2;
    pub(crate) const CARD_STROKE_WIDTH_PX: f32 = 1.5;
    pub(crate) const CARD_CORNER_RADIUS_PX: u8 = 4;
}

pub(crate) mod spacings {
    pub(crate) const SM_PX: f32 = 8.0;
    pub(crate) const MD_PX: f32 = 16.0;
    pub(crate) const LG_PX: f32 = 24.0;
}
