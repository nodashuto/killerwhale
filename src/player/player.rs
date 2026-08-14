use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Player systems
	app.add_systems(Startup, player_plugin_loaded);
    }
}

fn player_plugin_loaded() {
    println!("player plugin is loaded");
}
