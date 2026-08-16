use crate::app::app_state::AppState;
use crate::app::game::Game;
use crate::domain::card::Card;
use crate::domain::zone::ZoneId;

/// Events raised by the UI and handled by the application layer.
pub enum AppEvent {
    StartNewGame,
    Quit,
    ReturnToMenu,
    CardDropped(DropIntent),
}

/// Effects returned by the application layer for the UI to execute.
pub enum AppEffect {
    None,
    QuitApp,
}

/// High-level intent describing a dropped card action coming from the UI.
pub enum DropIntent {
    DiscardFromZone(ZoneId),
    PlaceFromTray { slot: usize, card: Card, zone: ZoneId },
    MoveZone { from: ZoneId, to: ZoneId },
}

#[derive(Default)]
pub struct App {
    state: AppState,
}

impl App {
    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn handle_event(&mut self, event: AppEvent) -> AppEffect {
        match event {
            AppEvent::StartNewGame => {
                self.state = AppState::Playing(Box::new(Game::new()));
                AppEffect::None
            }
            AppEvent::Quit => AppEffect::QuitApp,
            AppEvent::ReturnToMenu => {
                self.state = AppState::Menu;
                AppEffect::None
            }
            AppEvent::CardDropped(intent) => {
                match intent {
                    DropIntent::DiscardFromZone(from) => {
                        if let AppState::Playing(game) = &mut self.state {
                            game.discard_card(from);
                        }
                    }
                    DropIntent::PlaceFromTray { slot, card, zone } => {
                        if let AppState::Playing(game) = &mut self.state {
                            game.place_from_tray(slot, card, zone);
                        }
                    }
                    DropIntent::MoveZone { from, to } => {
                        if let AppState::Playing(game) = &mut self.state {
                            game.move_card(from, to);
                        }
                    }
                }

                AppEffect::None
            }
        }
    }
}
