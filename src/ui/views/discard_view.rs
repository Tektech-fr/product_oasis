use eframe::egui::{Align2, Color32, FontId, Rect, Stroke, StrokeKind, Ui};

use crate::domain::discard::Discard;

pub fn draw(ui: &Ui, rect: Rect, discard: &Discard) {
    let painter = ui.painter().with_clip_rect(rect);
    painter.rect_filled(rect, 4.0, Color32::from_gray(45));
    painter.rect_stroke(
        rect,
        4.0,
        Stroke::new(1.0, Color32::from_black_alpha(90)),
        StrokeKind::Inside,
    );
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        format!("Défausse ({})", discard.cards().len()),
        FontId::proportional(14.0),
        Color32::WHITE,
    );
}
