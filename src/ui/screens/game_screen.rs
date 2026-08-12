use eframe::egui::{Rect, Ui, pos2};

use crate::app::controller::{AppEvent, DropIntent};
use crate::app::game::Game;
use crate::domain::zone::BOARD_ASPECT;
use crate::ui::views::drag_state::DragState;
use crate::ui::views::drag_types::CardOrigin;
use crate::ui::views::geom_view::{fit_aspect, nearest_zone};
use crate::ui::views::theme::layout::{BOARD_MARGIN, DISCARD_HEIGHT_FRACTION, SIDE_WIDTH_FRACTION};
use crate::ui::views::{board_view, discard_view, tray_view};

pub fn show(ui: &mut Ui, game: &Game, drag: &mut DragState) -> Option<AppEvent> {
    let screen = ui.available_rect_before_wrap();

    let side_width = screen.width() * SIDE_WIDTH_FRACTION;
    let side_rect = Rect::from_min_max(pos2(screen.right() - side_width, screen.top()), screen.max);
    let board_area = Rect::from_min_max(screen.min, pos2(side_rect.left(), screen.bottom()))
        .shrink(BOARD_MARGIN);
    let board_rect = fit_aspect(board_area, BOARD_ASPECT);

    let discard_height = side_rect.height() * DISCARD_HEIGHT_FRACTION;
    let discard_rect = Rect::from_min_max(
        pos2(side_rect.left(), side_rect.bottom() - discard_height),
        side_rect.max,
    );
    let tray_rect = Rect::from_min_max(side_rect.min, pos2(side_rect.right(), discard_rect.top()));

    let board_drop = board_view::draw(ui, board_rect, &game.board, drag);
    let tray_drop = tray_view::draw(ui, tray_rect, board_rect, &game.tray, drag);
    discard_view::draw(ui, discard_rect, &game.discard);
    // Painted last so the dragged card always sits on top of everything else.
    drag.draw_floating(ui);

    let event = board_drop.or(tray_drop)?;
    if discard_rect.contains(event.drop_pos) {
        if let CardOrigin::Zone(from) = event.origin {
            return Some(AppEvent::CardDropped(DropIntent::DiscardFromZone(from)));
        }
        return None;
    }

    let target = nearest_zone(board_rect, event.drop_pos);
    match event.origin {
        CardOrigin::TraySlot(i) => Some(AppEvent::CardDropped(DropIntent::PlaceFromTray {
            slot: i,
            card: event.card,
            zone: target,
        })),
        CardOrigin::Zone(from) => Some(AppEvent::CardDropped(DropIntent::MoveZone {
            from,
            to: target,
        })),
    }
}
