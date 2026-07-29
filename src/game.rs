use bevy::{
    color::palettes::basic::PURPLE,
    prelude::*,
};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
	app.add_systems(Startup, setup);
	
    }
        // Register your systems here
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}
