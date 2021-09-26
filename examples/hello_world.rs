use bevy::prelude::*;
use template_lib::hello_world;

fn main() {
    App::build().add_system(hello_world.system()).run();
}
