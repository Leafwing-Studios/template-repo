use bevy::prelude::*;

pub mod utils;

pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(hello_world.system())
            .add_system(count_up.system());
    }
}

fn hello_world() {
    println!("Hello, World!");
}

#[derive(Default)]
struct Counter(u64);

fn count_up(mut counter: Local<Counter>) {
    counter.0 += 1;

    println!("{}", counter.0);
}
