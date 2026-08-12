use eframe::egui::{Align2, Color32, FontId, Rect, Stroke, StrokeKind, Ui, vec2};

use crate::domain::zone::ZoneId;
use crate::ui::views::theme::{readable_text_color, zone_color};

pub fn draw(ui: &Ui, rect: Rect, zone: ZoneId) {
    let painter = ui.painter().with_clip_rect(rect);
    let color = zone_color(zone);
    painter.rect_filled(rect, 4.0, color);
    painter.rect_stroke(
        rect,
        4.0,
        Stroke::new(1.0, Color32::from_black_alpha(90)),
        StrokeKind::Inside,
    );
    painter.text(
        rect.left_top() + vec2(6.0, 4.0),
        Align2::LEFT_TOP,
        zone.label(),
        FontId::proportional(12.0),
        readable_text_color(color),
    );
}
