use eframe::egui;

use crate::app::controller::{App as Application, AppEffect, AppEvent};
use crate::app::app_state::AppState;
use crate::ui::screens::game_screen;
use crate::ui::views::drag_state::DragState;
use crate::ui::views::menu_view::{self, MenuAction};

#[derive(Default)]
pub struct OasisApp {
    app: Application,
    drag: DragState,
    pending_event: Option<AppEvent>,
}

impl OasisApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for OasisApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(event) = self.pending_event.take() {
            match self.app.handle_event(event) {
                AppEffect::QuitApp => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                AppEffect::None => {}
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match self.app.state() {
            AppState::Menu => {
                if let Some(menu_action) = menu_view::show(ui) {
                    self.pending_event = Some(match menu_action {
                        MenuAction::NewGame => AppEvent::StartNewGame,
                        MenuAction::Quit => AppEvent::Quit,
                    });
                }
            }
            AppState::Playing(game) => {
                if let Some(ev) = game_screen::show(ui, game, &mut self.drag) {
                    self.pending_event = Some(ev);
                }
            }
        }
    }
}
