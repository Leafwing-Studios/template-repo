use bevy::prelude::*;

fn main() {
    App::build().add_system(hello_world.system()).run();
}

/// A system that says hello to the world
fn hello_world() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
