use eframe::egui::{Align2, FontId, Painter, Rect, Stroke, StrokeKind};

use crate::domain::card::Card;
use crate::ui::views::theme::{self, card_color, readable_text_color};

pub fn draw(painter: &Painter, rect: Rect, card: &Card) {
    let color = card_color(card.kind);
    painter.rect_filled(rect, 6.0, color);
    painter.rect_stroke(
        rect,
        6.0,
        Stroke::new(1.5, theme::colors::STROKES),
        StrokeKind::Inside,
    );
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        &card.name,
        FontId::proportional(14.0),
        readable_text_color(color),
    );
}
