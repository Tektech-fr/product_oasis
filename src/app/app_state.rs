use super::game::Game;

#[derive(Default)]
pub enum AppState {
    #[default]
    Menu,
    Playing(Box<Game>),
}
