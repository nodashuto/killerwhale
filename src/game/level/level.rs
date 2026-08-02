use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
use bevy::{
    color::palettes::tailwind,
    color::palettes::*,
    light::DirectionalLightTexture,
    camera::visibility::RenderLayers, 
};



use crate::game::target::target::{Target, TargetPlugin, Health, spawn_target};
use light_consts::lux::AMBIENT_DAYLIGHT;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
	app.add_systems(Startup, setup_physics);
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
    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone()), Transform::from_xyz(0.0, 0.1, 0.0),));

    // spawn block 
    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
	RigidBody::Fixed,
        Collider::cuboid(2.0, 0.5, 1.0),
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
	    color: Color::from(tailwind::NEUTRAL_500),
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

    
    spawn_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.5, -5.0),
    );
    spawn_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.5, -6.0),
    );

    let mesh3 = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    // spawn block 
    commands.spawn((
        Mesh3d(mesh3),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(30.0, 1.0, 30.0),
	RigidBody::Fixed,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));
}

// https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier3d/examples/boxes3.rs

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 100.1;
    let ground_height = 0.5;

    commands.spawn((
        Transform::from_xyz(0.0, -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    /*
     * Second Ground added myself
     */
    
    let second_ground_size = 500.1;
    let second_ground_height = 0.5;

    let second_ground_y = 100.1 + second_ground_height;

    commands.spawn((
        Transform::from_xyz(0.0, -second_ground_y, 0.0),
        Collider::cuboid(second_ground_size, second_ground_height, second_ground_size),
    ));

        

    /*
     * Create the cubes
     */
    let num = 8;
    let rad = 1.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
    let mut color = 0;
    let colors = [
        Hsla::hsl(220.0, 1.0, 0.3),
        Hsla::hsl(180.0, 1.0, 0.3),
        Hsla::hsl(260.0, 1.0, 0.7),
    ];

    for j in 0usize..20 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;
                color += 1;

                commands
                    .spawn(Transform::from_rotation(Quat::from_rotation_x(0.2)))
                    .with_children(|child| {
                        child.spawn((
                            Transform::from_xyz(x, y, z),
                            RigidBody::Dynamic,
                            Collider::cuboid(rad, rad, rad),
                            ColliderDebugColor(colors[color % 3]),
                        ));
                    });
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}
