use eframe::egui::{Pos2, Rect, Vec2, pos2, vec2};

use crate::domain::zone::{self, ZoneId};
use crate::ui::views::theme::layout::{CARD_ASPECT, CARD_MARGIN_IN_CELL, CELL_GAP};

pub fn fit_aspect(available: Rect, aspect: f32) -> Rect {
    let (w, h) = if available.width() / available.height() > aspect {
        (available.height() * aspect, available.height())
    } else {
        (available.width(), available.width() / aspect)
    };
    Rect::from_center_size(available.center(), vec2(w, h))
}

pub fn card_size_px(board_rect: Rect) -> Vec2 {
    let cell_w = board_rect.width() / zone::GRID_COLS as f32;
    let cell_h = board_rect.height() / zone::GRID_ROWS as f32;
    let unit_cell =
        Rect::from_min_size(board_rect.min, vec2(cell_w, cell_h)).shrink(CARD_MARGIN_IN_CELL);
    fit_aspect(unit_cell, CARD_ASPECT).size()
}

pub fn zone_rects(board_rect: Rect) -> impl Iterator<Item = (ZoneId, Rect)> {
    let cell_w = board_rect.width() / zone::GRID_COLS as f32;
    let cell_h = board_rect.height() / zone::GRID_ROWS as f32;
    zone::grid_cells().map(move |cell| {
        let min = pos2(
            board_rect.left() + cell.col as f32 * cell_w,
            board_rect.top() + cell.row as f32 * cell_h,
        );
        let size = vec2(cell_w * cell.col_span as f32, cell_h);
        (
            cell.zone,
            Rect::from_min_size(min, size).shrink(CELL_GAP / 2.0),
        )
    })
}

pub fn nearest_zone(board_rect: Rect, point: Pos2) -> ZoneId {
    zone_rects(board_rect)
        .min_by(|(_, a), (_, b)| {
            (a.center() - point)
                .length_sq()
                .total_cmp(&(b.center() - point).length_sq())
        })
        .map(|(zone, _)| zone)
        .expect("grid_cells() is never empty")
}
