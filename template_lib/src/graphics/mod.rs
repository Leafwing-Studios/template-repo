use bevy::prelude::Plugin;

use self::lighting::LightingPlugin;

mod lighting;

/// Adds game logic for rendering the game world. 
/// 
/// This should only contain logic to render and the game simulation should run without this.
pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LightingPlugin);
    }
}


// fn render_terrain