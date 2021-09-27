use bevy::prelude::*;
use bevy::window::WindowMode;
use template_lib::*;

fn main() {
    App::build()
        // Standard Bevy functionality
        .add_plugins(DefaultPlugins)
        // Configure the window settings
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: "Template".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        // Add plugins here
        .add_plugin(HelloWorldPlugin)
        .run();
}
