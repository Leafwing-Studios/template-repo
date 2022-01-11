use bevy::prelude::*;
use template_lib::HelloWorldPlugin;

fn main() {
    App::new().add_plugin(HelloWorldPlugin).run();
}
