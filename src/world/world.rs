use bevy::prelude::*;
use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind, light::CascadeShadowConfigBuilder,
    light::NotShadowCaster,
};
use bevy_rapier3d::prelude::*;

use std::f32::consts::PI;
// use std::f32::consts::TAU;

// use crate::shootingtarget::{
//     spawn_shooting_target,
//     // ShootingTarget,
//     ShootingTargetPlugin,
// };

// use crate::shootingtarget::ShootingTargetPlugin;

// use crate::Health;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierDebugRenderPlugin::default()); //activate gismo
        //app.add_plugins(ShootingTargetPlugin);
        // app.insert_resource(ClearColor(Color::srgb(
        //     226.0 / 255.0,
        //     237.0 / 255.0,
        //     238.0 / 255.0,
        // )));
        app.insert_resource(ClearColor(Color::srgb(
            152.0 / 255.0,
            192.0 / 255.0,
            217.0 / 255.0,
        )));
        // app.insert_resource(ClearColor(Color::BLACK));
        app.add_systems(Startup, spawn_world_model);
        app.add_systems(Startup, spawn_lights);
        //app.add_systems(Startup, spawn_stairs);
        // app.add_systems(Startup, spawn_cube);

	//app.add_systems(Update, draw_grid);

	// app.add_systems(Startup, load_lamp).add_systems(Update, spawn_lamp_collider);
        app.add_systems(Startup, load_map)
            .add_systems(Update, spawn_map_collider);
        //app.add_systems(Startup, spawn_mesh);
        //app.add_systems(Startup, spawn_wall);
    }
}

// draw grid on floor
fn draw_grid(mut gizmos: Gizmos) {
    gizmos
        .grid(
            Quat::from_rotation_x(PI / 2.),
            UVec2::splat(40),
            Vec2::new(1., 1.),
            Color::linear_rgb(0.7, 0., 0.4),
        )
        .outer_edges();
}

fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    let size = 100.0;
    let ground = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(size)));
    let _cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let _material = materials.add(Color::WHITE);

    // let grass = materials.add(Color::srgb(0.4, 1.0, 0.2));

    let _wood = materials.add(Color::srgb(145.0 / 255.0, 117.0 / 255.0, 77.0 / 255.0));

    // let sand = materials.add(Color::srgb(234.0 / 255.0, 225.0 / 255.0, 208.0 / 255.0));

    let transparent = materials.add(StandardMaterial {
        base_color: Color::srgb(
            234.0 / 255.0,
            225.0 / 255.0,
            208.0 / 255.0,
            //1.0,
        ),
        //alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let _house = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.

    /*
     * floor(ground)
     */
    commands.spawn((
        Mesh3d(ground),
        MeshMaterial3d(transparent.clone()),
        RigidBody::Fixed,
        Collider::cuboid(size, 0.1, size),
        Transform::from_xyz(0.0, -0.06, 0.0),
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

    // // spawn shooting target
    // spawn_shooting_target(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(0.0, 0.95, -10.0),
    // );

    // spawn_shooting_target(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(0.5, 0.95, -10.0),
    // );

    // spawn_shooting_target(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(1.0, 0.95, -10.0),
    // );

    // spawn_shooting_target(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(-1.0, 0.95, -10.0),
    // );

    // spawn_shooting_target(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(-0.5, 0.95, -10.0),
    // );

    // spawn_shooting_target(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(1.5, 0.95, -10.0),
    // );

    // spawn_shooting_target(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(-1.4, 0.95, -10.0),
    // );

    // /*
    //  * spawns table
    //  */
    // spawn_table(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     Vec3::new(0.0, -0.2, -10.0),
    // );

    /*
     * spawns ground texture
     */

    // commands.spawn((
    //     //Mesh3d(asset_server.load("models/tutorial-texture-wood.glb#Mesh0/Primitive0")),
    //     WorldAssetRoot(
    //         asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/ground_plane_002.glb")), //
    //     ),
    //     Transform::from_xyz(0.0, -0.05, 40.0),
    // ));

    // commands.spawn((
    //     RigidBody::Fixed,
    //     Transform::from_xyz(12.0, 0.0, 0.0),
    //     Collider::from_bevy_mesh(
    //         meshes.get(&mesh_handle).expect("mesh load error!!!!!!!!!"),
    //         &ComputedColliderShape::TriMesh(TriMeshFlags::all()),
    //     )
    //     .unwrap(),
    // ));

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

// #[derive(Resource)]
// struct LampAsset {
//     mesh: Handle<Mesh>,
//     spawned: bool,
// }

// fn load_lamp(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     let mesh = asset_server.load::<Mesh>(
//         "models/lamp-0001.glb#Mesh0/Primitive0"
//     );

//     commands.insert_resource(LampAsset {
//         mesh,
//         spawned: false,
//     });
// }

// fn spawn_lamp_collider(
//     mut commands: Commands,
//     mut lamp: ResMut<LampAsset>,
//     meshes: Res<Assets<Mesh>>,
// ) {
//     if lamp.spawned {
//         return;
//     }

//     let Some(mesh) = meshes.get(&lamp.mesh) else {
//         return;
//     };

//     let collider = Collider::from_bevy_mesh(
//         mesh,
//         &ComputedColliderShape::TriMesh(TriMeshFlags::all()),
//     )
//     .expect("Failed to create lamp collider");

//     commands.spawn((
//         RigidBody::Fixed,
//         Transform::from_xyz(12.0, 5.0, -12.0),
//         collider,
//     ));

//     lamp.spawned = true;
// }

#[derive(Resource)]
struct MapAsset {
    mesh: Handle<Mesh>,
    scene: Handle<WorldAsset>,
    spawned: bool,
}

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mesh = asset_server.load::<Mesh>("maps/mp-0001.glb#Mesh0/Primitive0");

    let scene =
        asset_server.load::<WorldAsset>(GltfAssetLabel::Scene(0).from_asset("maps/mp-0001.glb"));

    commands.insert_resource(MapAsset {
        mesh,
        spawned: false,
        scene,
    });
}

fn spawn_map_collider(
    mut commands: Commands,
    mut map: ResMut<MapAsset>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if map.spawned {
        return;
    }

    let Some(mesh) = meshes.get(&map.mesh) else {
        return;
    };

    let collider =
        Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh(TriMeshFlags::all()))
            .expect("Failed to create map collider");

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.4, 0.4),
        perceptual_roughness: 0.8,
	metallic: 1.0,
        ..default()
    });

    let origin = Vec3::new(0.0, 0.0, 0.0);

    let transform = Transform::from_xyz(0.0, 0.0, 0.0);

    

    // Spawn the visual GLB scene
    commands.spawn((
        WorldAssetRoot(map.scene.clone()),
       transform,
    ));

    // Spawn the physics collider
    commands.spawn((
        RigidBody::Fixed,
        transform,
        collider,
	// Visual mesh
        //Mesh3d(map.mesh.clone()),
	
        // Standard material
        //MeshMaterial3d(material),
	
    ));



    map.spawned = true;
}

fn _spawn_mesh(
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
        Transform::from_xyz(0.0, 1.0, 30.0),
    ));
}

// fn spawn_stairs(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut _materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     // Load the mesh from the GLB
//     let _mesh = meshes.add(Cuboid::new(2.0, 0.5, 1.0));

//     // commands.spawn((
//     //     Mesh3d(mesh.clone()),
//     //     MeshMaterial3d(materials.add(StandardMaterial {
//     //         base_color: Color::BLACK,
//     //         ..default()
//     //     })),
//     //     RigidBody::Fixed,
//     //     Collider::cuboid(2.0, 0.5, 1.0),
//     //     Transform::from_xyz(14.0, 0.0, 0.0),
//     // ));

//     let origin = Vec3::new(0.0, 0.0, 15.0);

//     commands.spawn((
//         //Mesh3d(asset_server.load("models/tutorial-texture-wood.glb#Mesh0/Primitive0")),
//         WorldAssetRoot(
//             asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Stairs-0001.glb")), // models/tutorial-texture-wood.glb
//         ),
//         //RigidBody::Fixed,
//         //Collider::cuboid(0.1, 0.2, 0.1),
//         Transform::from_translation(origin.clone()),
//     ));

//     commands.spawn((
//         RigidBody::Fixed,
//         Collider::cuboid(1.0, 0.1, 1.0),
//         Transform::from_translation(origin.clone()),
//     ));
// }

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

// // test spawn cube collider from vertices and indices
// fn spawn_cube(mut commands: Commands) {
//     let vertices = vec![
//         Vec3::new(-0.5, -0.5, -0.5),
//         Vec3::new(0.5, -0.5, -0.5),
//         Vec3::new(0.5, 0.5, -0.5),
//         Vec3::new(-0.5, 0.5, -0.5),
//         Vec3::new(-0.5, -0.5, 0.5),
//         Vec3::new(0.5, -0.5, 0.5),
//         Vec3::new(0.5, 0.5, 0.5),
//         Vec3::new(-0.5, 0.5, 0.5),
//     ];

//     let indices = vec![
//         [0, 2, 1],
//         [0, 3, 2],
//         [4, 5, 6],
//         [4, 6, 7],
//         [0, 1, 5],
//         [0, 5, 4],
//         [2, 3, 7],
//         [2, 7, 6],
//         [0, 4, 7],
//         [0, 7, 3],
//         [1, 2, 6],
//         [1, 6, 5],
//     ];

//     let collider =
//         Collider::trimesh_with_flags(vertices, indices, TriMeshFlags::FIX_INTERNAL_EDGES)
//             .expect("Failed to create cube trimesh collider");

//     commands.spawn((
//         RigidBody::Fixed,
//         collider,
//         Transform::from_xyz(-16.0, 1.0, -5.0),
//     ));
// }

fn _spawn_wall(
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

// pub fn spawn_table(
//     commands: &mut Commands,
//     meshes: &mut Assets<Mesh>,
//     materials: &mut Assets<StandardMaterial>,
//     position: Vec3,
// ) {
//     let depth = 0.1;
//     let thickness = 0.1;
//     let width = 3.0;
//     let mesh = meshes.add(Cuboid::new(width, thickness, depth));

//     let material = materials.add(StandardMaterial {
//         base_color: Color::srgb(99.0 / 255.0, 73.0 / 255.0, 43.0 / 255.0),
//         ..default()
//     });

//     let leg_length = 1.0;

//     let leg_mesh = meshes.add(Cuboid::new(depth, leg_length, depth));

//     let top_pos = Vec3::new(
//         position.x,
//         position.y + leg_length + thickness / 2.0,
//         position.z,
//     );

//     //spawn top
//     commands.spawn((
//         Mesh3d(mesh.clone()),
//         MeshMaterial3d(material.clone()),
//         Transform::from_translation(top_pos),
//         RigidBody::Fixed,
//         Collider::cuboid(width / 2.0, thickness / 2.0, depth / 2.0),
//     ));

//     //spawn middle
//     commands.spawn((
//         Mesh3d(mesh.clone()),
//         MeshMaterial3d(material.clone()),
//         Transform::from_translation(Vec3::new(
//             position.x,
//             position.y + leg_length / 2.0 + thickness,
//             position.z,
//         )),
//         RigidBody::Fixed,
//         Collider::cuboid(width / 2.0, thickness / 2.0, depth / 2.0),
//     ));

//     //spawn one leg
//     commands.spawn((
//         Mesh3d(leg_mesh.clone()),
//         MeshMaterial3d(material.clone()),
//         Transform::from_translation(Vec3::new(
//             position.x + width / 2.0 + thickness / 2.0,
//             position.y + leg_length / 2.0 + thickness,
//             position.z,
//         )),
//         RigidBody::Fixed,
//         Collider::cuboid(depth / 2.0, leg_length / 2.0, depth / 2.0),
//     ));

//     //spawn the other leg
//     commands.spawn((
//         Mesh3d(leg_mesh.clone()),
//         MeshMaterial3d(material.clone()),
//         Transform::from_translation(Vec3::new(
//             position.x - width / 2.0 - thickness / 2.0,
//             position.y + leg_length / 2.0 + thickness,
//             position.z,
//         )),
//         RigidBody::Fixed,
//         Collider::cuboid(depth / 2.0, leg_length / 2.0, depth / 2.0),
//     ));
// }

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

// fn spawn_lights(mut commands: Commands) {
//     // Spawn Global Light
//     // commands.spawn((
//     //     Transform::from_xyz(-50., 500.0, 100.)
//     //         .looking_at(Vec3::ZERO, Vec3::Y)
//     //         .with_scale(Vec3::splat(2.)),
//     //     DirectionalLight {
//     //         color: Color::from(tailwind::NEUTRAL_500),
//     //         illuminance: AMBIENT_DAYLIGHT,
//     //         shadow_maps_enabled: true,
//     //         ..default()
//     //     },
//     //     Visibility::Visible,
//     // ));

//     // Spawn PointLight
//     commands.spawn((
//         PointLight {
//             color: Color::from(tailwind::NEUTRAL_800), // NEUTRAL_800, EMERALD_600
//             shadow_maps_enabled: true,
//             ..default()
//         },
//         Transform::from_xyz(-2.0, 2.0, -0.75),
//         // The light source illuminates both the world model and the view model.
//         RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
//     ));
// }

// spawn light  withbetter in-game performance
fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0_f32.to_radians(),
            -30.0_f32.to_radians(),
            0.0,
        )),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 10.0,
            maximum_distance: 40.0,
            ..default()
        }
        .build(),
    ));
}
