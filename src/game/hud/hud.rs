use bevy::prelude::*;

use super::crosshair;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, crosshair::spawn_crosshair);
    }
}
