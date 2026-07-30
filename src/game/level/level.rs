use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
use bevy::{
    color::palettes::tailwind,
    light::DirectionalLightTexture,
};

use light_consts::lux::AMBIENT_DAYLIGHT;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
    }
}

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn init_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    // spawn floor
    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone())));

    // spawn block 
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material),
        Transform::from_xyz(0.75, 1.75, 0.0),
    ));

    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
	    intensity: 000_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
  
    ));

    commands.spawn((
        Transform::from_xyz(-10., 500.0, 10.)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(Vec3::splat(2.)),
        DirectionalLight {
            illuminance: AMBIENT_DAYLIGHT,
            shadow_maps_enabled: false,
            ..default()
        },
        Visibility::Visible,
    ));
    

}
