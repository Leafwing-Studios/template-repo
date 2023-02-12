//! Everything needed to run the main game logic

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};

/// Set the game state to align systems with their respective runtimes
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn main() {
    App::new().add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("3D Bevy Template"),
                    ..default()
                }),
                ..default()
            })
            // Work around for https://github.com/bevyengine/bevy/issues/7620
            .set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    backends:Some(Backends::PRIMARY),
                    ..Default::default()
                },
            }),
        ).run();
}
