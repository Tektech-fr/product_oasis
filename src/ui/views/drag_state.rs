use eframe::egui::{Id, Rect, Sense, Ui, Vec2};

use crate::domain::card::Card;
use crate::ui::views::card_view;
use crate::ui::views::drag_types::{CardOrigin, DraggedCard, DropEvent};

#[derive(Default)]
pub struct DragState {
    current: Option<DraggedCard>,
}

impl DragState {
    pub fn interact(
        &mut self,
        ui: &Ui,
        id: Id,
        origin: CardOrigin,
        card: Card,
        home: Rect,
    ) -> Option<DropEvent> {
        if matches!(&self.current, Some(d) if d.origin != origin) {
            card_view::draw(ui.painter(), home, &card);
            return None;
        }

        let response = ui.interact(home, id, Sense::click_and_drag());

        if response.drag_started() {
            let grab = response
                .interact_pointer_pos()
                .map_or(Vec2::ZERO, |p| p - home.min);
            self.current = Some(DraggedCard {
                origin,
                card: card.clone(),
                grab_offset: grab,
                size: home.size(),
            });
        }

        if self.current.is_none() {
            card_view::draw(ui.painter(), home, &card);
            return None;
        }

        if response.drag_stopped() {
            let dragged = self.current.take().expect("checked above");
            return response
                .interact_pointer_pos()
                .map(|p| Rect::from_min_size(p - dragged.grab_offset, dragged.size).center())
                .map(|drop_pos| DropEvent {
                    origin,
                    card: dragged.card,
                    drop_pos,
                });
        }

        None
    }

    pub fn draw_floating(&self, ui: &Ui) {
        let Some(dragged) = &self.current else { return };
        let Some(pointer) = ui.input(|i| i.pointer.interact_pos()) else {
            return;
        };
        let rect = Rect::from_min_size(pointer - dragged.grab_offset, dragged.size);
        card_view::draw(ui.painter(), rect, &dragged.card);
    }
}
