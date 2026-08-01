use bevy::prelude::*;

use super::crosshair;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_text, crosshair::spawn_crosshair));
    }
}

fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(12),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Debug Build\n",
            "bevy = 0.19.0 \n",
            "bevy_rapier3d = 0.35.0 "
        )));
}
