use eframe::egui::{Id, Rect, Ui};

use crate::domain::board::Board;
use crate::ui::views::drag_state::DragState;
use crate::ui::views::drag_types::{CardOrigin, DropEvent};
use crate::ui::views::geom_view::{card_size_px, zone_rects};
use crate::ui::views::zone_view;

pub fn draw(ui: &Ui, board_rect: Rect, board: &Board, drag: &mut DragState) -> Option<DropEvent> {
    let card_size = card_size_px(board_rect);
    let mut drop = None;

    for (id, rect) in zone_rects(board_rect) {
        zone_view::draw(ui, rect, id);

        if let Some(card) = board.card_at(id) {
            let card_rect = Rect::from_center_size(rect.center(), card_size);
            let widget_id = Id::new("board_card").with(id);
            if let Some(event) =
                drag.interact(ui, widget_id, CardOrigin::Zone(id), card.clone(), card_rect)
            {
                drop = Some(event);
            }
        }
    }

    drop
}
