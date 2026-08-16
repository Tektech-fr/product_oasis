use eframe::egui::{Align2, Color32, FontId, Rect, Stroke, StrokeKind, Ui};

use crate::domain::zone::ZoneId;
use crate::ui::views::theme::{readable_text_color, zone_color};

pub fn draw(ui: &Ui, rect: Rect, zone: ZoneId) {
    let painter = ui.painter().with_clip_rect(rect);
    let color = Color32::from_black_alpha(90);
    painter.rect_filled(rect, 4.0, color);
    painter.rect_stroke(
        rect,
        4.0,
        Stroke::new(4.0, zone_color(zone)),
        StrokeKind::Inside,
    );
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        zone.label(),
        FontId::proportional(12.0),
        readable_text_color(color),
    );
}
