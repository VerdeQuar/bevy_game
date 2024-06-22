use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(Setup, setup)
        .run()
}

fn setup() {}
