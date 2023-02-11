//! Everything needed to run the main game logic 

use bevy::prelude::*;

/// This lets us set a state to select from for system scheduling.  
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    // TODO: Choose desired default
    MainMenu,
    #[default]
    Playing,
    PauseMenu,

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .run();
}