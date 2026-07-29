use bevy::prelude::*;

use bevy_rapier3d::prelude::*;


pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });


}
