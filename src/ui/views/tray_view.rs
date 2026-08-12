use eframe::egui::{Color32, Id, Rect, Ui, pos2};

use crate::domain::tray::Tray;
use crate::ui::views::drag_state::DragState;
use crate::ui::views::drag_types::{CardOrigin, DropEvent};
use crate::ui::views::geom_view::card_size_px;

const SLOT_GAP: f32 = 18.0;
const SLOT_MARGIN: f32 = 20.0;

pub fn draw(
    ui: &Ui,
    tray_rect: Rect,
    board_rect: Rect,
    tray: &Tray,
    drag: &mut DragState,
) -> Option<DropEvent> {
    ui.painter()
        .rect_filled(tray_rect, 6.0, Color32::from_gray(32));

    let card_size = card_size_px(board_rect);
    let mut drop = None;

    for (i, card) in tray.slots() {
        let top = tray_rect.top() + SLOT_MARGIN + i as f32 * (card_size.y + SLOT_GAP);
        let home = Rect::from_min_size(
            pos2(tray_rect.center().x - card_size.x / 2.0, top),
            card_size,
        );
        let id = Id::new("tray_slot").with(i);
        if let Some(event) = drag.interact(ui, id, CardOrigin::TraySlot(i), card.clone(), home) {
            drop = Some(event);
        }
    }

    drop
}
