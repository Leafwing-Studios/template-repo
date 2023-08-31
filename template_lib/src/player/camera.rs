//! Code needed to run the game camera

use bevy::prelude::*;

/// Camera logic
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

/// Spawn the player camera
fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
