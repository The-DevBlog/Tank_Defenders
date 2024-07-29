mod game;
mod game_over;

use bevy::app::App;
use bevy::prelude::*;
use game::GamePlugin;
use game_over::GameOverPlugin;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((GamePlugin, GameOverPlugin))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Game,
    GameOver,
}
