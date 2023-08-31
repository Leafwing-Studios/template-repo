//! Code that connects player character logic
use bevy::prelude::{App, Plugin};

pub mod camera;

/// Create a plugin to use with the main game logic
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin);
    }
}
