use bevy::prelude::*;


use game::GamePlugin;

// This line tells the compiler to include the code it finds in src/game.rs
pub mod game;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}



