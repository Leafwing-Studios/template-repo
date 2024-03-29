//! Lights and lighting.
// Modified from Leafwing *Emergence*

use bevy::prelude::*;

/// Handles all lighting logic
pub(super) struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            brightness: 1.,
            color: Color::WHITE,
        })
        .add_startup_system(spawn_sun);
    }
}

/// Spawns a directional light source to illuminate the scene
fn spawn_sun(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(30., 100., 30.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
