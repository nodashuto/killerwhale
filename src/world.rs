use bevy::prelude::*;
use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind, light::CascadeShadowConfigBuilder,
    light::NotShadowCaster,
};
use bevy_rapier3d::prelude::*;

use std::f32::consts::TAU;

use crate::shootingtarget::{spawn_shooting_target, ShootingTarget, ShootingTargetPlugin};

// use crate::shootingtarget::ShootingTargetPlugin;

use crate::Health;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierDebugRenderPlugin::default()); //activate gismo
        app.add_plugins(ShootingTargetPlugin);
        app.insert_resource(ClearColor(Color::srgb(
            226.0 / 255.0,
            237.0 / 255.0,
            238.0 / 255.0,
        )));
        app.add_systems(Startup, spawn_world_model);
        app.add_systems(Startup, spawn_lights);
	app.add_systems(Startup, spawn_stairs);
        //app.add_systems(Startup, spawn_mesh);
        //app.add_systems(Startup, spawn_wall);
    }
}

fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(64.0)));
    let _cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let _material = materials.add(Color::WHITE);

    // let grass = materials.add(Color::srgb(0.4, 1.0, 0.2));

    let _wood = materials.add(Color::srgb(145.0 / 255.0, 117.0 / 255.0, 77.0 / 255.0));

    let sand = materials.add(Color::srgb(234.0 / 255.0, 225.0 / 255.0, 208.0 / 255.0));

    let _house = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.

    /*
     * floor(ground)
     */
    commands.spawn((
        Mesh3d(ground),
        MeshMaterial3d(sand.clone()),
        RigidBody::Fixed,
        Collider::cuboid(64.0, 0.1, 64.0),
        Transform::from_xyz(0.0, -0.1, 0.0),
    ));

    // commands.spawn((
    //     Mesh3d(cube.clone()),
    //     MeshMaterial3d(material.clone()),
    //     Transform::from_xyz(0.0, 0.25, -3.0),
    // ));

    // commands.spawn((
    // 	Mesh3d(house.clone()),
    // 	MeshMaterial3d(material.clone()),
    // 	RigidBody::Fixed,
    //     Collider::cuboid(0.5, 0.5, 0.5),
    // 	Transform::from_xyz(4.0, -0.4, 4.0),
    // ));

    // commands.spawn((
    // 	Mesh3d(house.clone()),
    // 	MeshMaterial3d(material.clone()),
    // 	RigidBody::Fixed,
    //     Collider::cuboid(0.5, 0.5, 0.5),
    // 	Transform::from_xyz(4.0, -0.2, 3.0),
    // ));

    // wall x z ++

    // let wall = meshes.add(Cuboid::new(2.0, 2.0, 0.1));
    // commands.spawn((
    // 	Mesh3d(wall.clone()),
    // 	MeshMaterial3d(material.clone()),
    // 	RigidBody::Fixed,
    //     Collider::cuboid(1.0 , 1.0, 0.05),
    // 	Transform::from_xyz(3.0, 1.0, -4.0),
    // 	));

    // spawn shooting target
    spawn_shooting_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.95, -10.0),
    );

    spawn_shooting_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.5, 0.95, -10.0),
    );

    spawn_shooting_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(1.0, 0.95, -10.0),
    );

    spawn_shooting_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-1.0, 0.95, -10.0),
    );

    spawn_shooting_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-0.5, 0.95, -10.0),
    );

    spawn_shooting_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(1.5, 0.95, -10.0),
    );

    spawn_shooting_target(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-1.4, 0.95, -10.0),
    );

    /*
     * spawns table
     */
    spawn_table(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, -0.2, -10.0),
    );

    /*
     * spawn stair
     */
    // spawn_stair(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(0.0, 0.0, 10.0),
    // );
    

    //  commands.spawn((
    //     DirectionalLight {
    //         illuminance: light_consts::lux::OVERCAST_DAY,
    //         shadow_maps_enabled: true,
    //         ..default()
    //     },
    //     Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_rotation_x(-PI / 4.),
    //         ..default()
    //     },
    //     // The default cascade config is designed to handle large scenes.
    //     // As this example has a much smaller world, we can tighten the shadow
    //     // bounds for better visual quality.
    //     CascadeShadowConfigBuilder {
    //         first_cascade_far_bound: 4.0,
    //         maximum_distance: 10.0,
    //         ..default()
    //     }
    //     .build(),
    // ));
}

fn spawn_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load the mesh from the GLB
    let mesh = meshes.add(Cuboid::new(2.0, 0.5, 1.0));

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::BLACK,
            ..default()
        })),
        RigidBody::Fixed,
        Collider::cuboid(2.0, 0.5, 1.0),
        Transform::from_xyz(14.0, 0.0, 0.0),
    ));

    commands.spawn((
        //Mesh3d(asset_server.load("models/tutorial-texture-wood.glb#Mesh0/Primitive0")),
        WorldAssetRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("models/tutorial-texture-wood.glb")), // 
        ),
        RigidBody::Fixed,
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_xyz(0.0, 1.0, -30.0),
    ));
}


fn spawn_stairs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load the mesh from the GLB
    let mesh = meshes.add(Cuboid::new(2.0, 0.5, 1.0));

    // commands.spawn((
    //     Mesh3d(mesh.clone()),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::BLACK,
    //         ..default()
    //     })),
    //     RigidBody::Fixed,
    //     Collider::cuboid(2.0, 0.5, 1.0),
    //     Transform::from_xyz(14.0, 0.0, 0.0),
    // ));

    let origin = Vec3::new(0.0, 0.0, 15.0);

    commands.spawn((
        //Mesh3d(asset_server.load("models/tutorial-texture-wood.glb#Mesh0/Primitive0")),
        WorldAssetRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("models/Stairs-0001.glb")), // models/tutorial-texture-wood.glb
        ),
        RigidBody::Fixed,
        Collider::cuboid(0.1, 0.2, 0.1),
        Transform::from_translation(origin.clone()),
    ));

    commands.spawn((
	RigidBody::Fixed,
	Collider::cuboid(1.0, 0.1, 1.0),
	Transform::from_translation(origin.clone()),
    ));
}


// fn spawn_stair(
//     mut commands: Commands,
//     mut _meshes: Assets<Mesh>,
//     mut _materials: Assets<StandardMaterial>,
//     asset_server: Res<AssetServer>, //position: Vec3,
// ) {
//     // 
//     commands.spawn((
// 	WorldAssetRoot(
//             asset_server
//                 .load(GltfAssetLabel::Scene(0).from_asset("models/Stairs-0001.glb")),
//         ),
//         RigidBody::Fixed,
//         Collider::cuboid(1.0, 1.0, 1.0),        
//         Transform::from_xyz(0.0, 0.0, 10.0),
//     ));
// }

fn spawn_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_width = 2.0;
    let wall_height = 2.0;
    let wall_depth = 0.1;

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(wall_width, wall_height, wall_depth))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),
        RigidBody::Fixed,
        Collider::cuboid(wall_width / 2.0, wall_height / 2.0, wall_depth / 2.0),
        Transform::from_xyz(-6.0, 1.0, 0.0)
            .with_rotation(Quat::from_rotation_y(0.25 * std::f32::consts::PI)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(wall_width, wall_height, wall_depth))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),
        RigidBody::Fixed,
        Collider::cuboid(wall_width / 2.0, wall_height / 2.0, wall_depth / 2.0),
        Transform::from_xyz(-18.0, 0.5, 0.0)
            .with_rotation(Quat::from_rotation_x(0.30 * std::f32::consts::PI)),
    ));
}

pub fn spawn_table(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    let depth = 0.1;
    let thickness = 0.1;
    let width = 3.0;
    let mesh = meshes.add(Cuboid::new(width, thickness, depth));

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(99.0 / 255.0, 73.0 / 255.0, 43.0 / 255.0),
        ..default()
    });

    let leg_length = 1.0;

    let leg_mesh = meshes.add(Cuboid::new(depth, leg_length, depth));

    let top_pos = Vec3::new(
        position.x,
        position.y + leg_length + thickness / 2.0,
        position.z,
    );

    //spawn top
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(top_pos),
        RigidBody::Fixed,
        Collider::cuboid(width / 2.0, thickness / 2.0, depth / 2.0),
    ));

    //spawn middle
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::new(
            position.x,
            position.y + leg_length / 2.0 + thickness,
            position.z,
        )),
        RigidBody::Fixed,
        Collider::cuboid(width / 2.0, thickness / 2.0, depth / 2.0),
    ));

    //spawn one leg
    commands.spawn((
        Mesh3d(leg_mesh.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::new(
            position.x + width / 2.0 + thickness / 2.0,
            position.y + leg_length / 2.0 + thickness,
            position.z,
        )),
        RigidBody::Fixed,
        Collider::cuboid(depth / 2.0, leg_length / 2.0, depth / 2.0),
    ));

    //spawn the other leg
    commands.spawn((
        Mesh3d(leg_mesh.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::new(
            position.x - width / 2.0 - thickness / 2.0,
            position.y + leg_length / 2.0 + thickness,
            position.z,
        )),
        RigidBody::Fixed,
        Collider::cuboid(depth / 2.0, leg_length / 2.0, depth / 2.0),
    ));
}

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn spawn_lights(mut commands: Commands) {
    // Spawn Global Light
    // commands.spawn((
    //     Transform::from_xyz(-50., 500.0, 100.)
    //         .looking_at(Vec3::ZERO, Vec3::Y)
    //         .with_scale(Vec3::splat(2.)),
    //     DirectionalLight {
    //         color: Color::from(tailwind::NEUTRAL_500),
    //         illuminance: AMBIENT_DAYLIGHT,
    //         shadow_maps_enabled: true,
    //         ..default()
    //     },
    //     Visibility::Visible,
    // ));

    // Spawn PointLight
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::NEUTRAL_800),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 2.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}
