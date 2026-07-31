use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
use bevy::{
    color::palettes::tailwind,
    color::palettes::*,
    light::DirectionalLightTexture,
    camera::visibility::RenderLayers, 
};

    
use light_consts::lux::AMBIENT_DAYLIGHT;

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
    asset_server: Res<AssetServer>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    // spawn floor
    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone()), Transform::from_xyz(0.0, -1.5, 0.0),));

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
        Transform::from_xyz(-50., 500.0, 100.)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .with_scale(Vec3::splat(2.)),
        DirectionalLight {
	    color: Color::from(tailwind::AMBER_100),
	    illuminance: AMBIENT_DAYLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        Visibility::Visible,

    ));

    //commands.spawn(WorldAssetRoot(asset_server.load(
      //  GltfAssetLabel::Scene(0).from_asset("models/sniper-0001.glb#Scene0"),
    //)));

    // Load the mesh from the GLB
    let mesh = asset_server.load("models/sniper-0001.glb#Mesh0/Primitive0");

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..default()
        })),
        Transform::default(),
    ));

    let mesh2 = asset_server.load("models/sniper-0001.glb#Mesh0/Primitive0");

    
    commands.spawn((
        Mesh3d(mesh2),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),

	Transform {
	    translation: Vec3::new(3.0, 0.0, 0.0),
	    rotation: Quat::from_rotation_y(1.57),
	    scale: Vec3::new(1.0, 1.0, 5.0),
	}
    ));


}
