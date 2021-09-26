use bevy::prelude::*;

fn main() {
    App::build().add_system(hello_world.system()).run();
}

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
