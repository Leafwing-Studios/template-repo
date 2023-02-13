//! Everything needed to run the main game logic

use bevy::prelude::*;

/// Set the game state to align systems with their respective runtimes
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Bevy 3D Template"),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(template_lib::player::PlayerPlugin)
        .add_plugin(template_lib::graphics::GraphicsPlugin)
        .run();
}
