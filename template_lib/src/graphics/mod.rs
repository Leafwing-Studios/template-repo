//! Logic for starting the graphics pipeline
use bevy::prelude::{App, Plugin};

use self::lighting::LightingPlugin;

mod lighting;

/// Adds game logic for rendering the game world.
///
/// This should only contain logic to render and the game simulation should run without this.
pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LightingPlugin);
    }
}

// fn render_terrain
