use std::f32::consts::FRAC_PI_2;

use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    light::NotShadowCaster, 
};


use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};






use light_consts::lux::AMBIENT_DAYLIGHT;

pub mod world;


const PLAYER_SPEED: f32 = 4.0;
const JUMP_IMPULSE: f32 = 3.5;
// const MOUSE_SENSITIVITY: f32 = 0.002;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Basic FPS".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(world::WorldPlugin)
        //.add_plugins(RapierDebugRenderPlugin::default()) // Uncomment for collider visualization
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.15)))
        //.add_systems(Startup, setup)
	// .add_systems(Startup, initial_grab_cursor)
	// .add_systems(Startup, initial_grab_on_player_spawn)
	// .add_systems(Update, player_look)
        // .add_systems(Update, player_movement)
	//.add_systems(Startup, setup) //needthis
	
        //.add_systems(Update, mouse_look)
    // .add_systems(Update, grab_mouse)
	.add_systems(
            Startup,
            (
                spawn_view_model,                
                spawn_lights,
                spawn_text,
            ),
        )
	.add_systems(Startup, initial_grab_cursor)
        .add_systems(Update, (move_player, change_fov))
	.add_systems(Update,player_movement) //needthis
	.add_systems(Update, cursor_grab)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct LookAngles {
    yaw: f32,
    pitch: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn((
        PointLight {
            intensity: 5000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0),
    ));

    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 1.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.6, 0.3))),
        Transform::from_xyz(0.0, -0.5, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(25.0, 0.5, 25.0),
    ));



    // Red box
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(9.0, -0.3, 6.0),
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
    ));

    // second Red box
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(9.0, 0.1, 5.0),
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
    ));

    // Blue Wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(-10.0, 0.0, -10.0),
        RigidBody::Fixed,
        Collider::cuboid(2.5, 2.5, 0.5),
    ));


    // Player
    commands.spawn((
        Player,
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 1.0))),
        Transform::from_xyz(0.0, 2.0, 0.0),
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.4),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
	KinematicCharacterController {
            ..KinematicCharacterController::default()
        },
        Damping {
            linear_damping: 2.0,
            angular_damping: 100.0,
        },
        Camera3d::default(),
    ));
}



/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    match primary_cursor_options.grab_mode {
        CursorGrabMode::None => {
            primary_cursor_options.grab_mode = CursorGrabMode::Confined;
            primary_cursor_options.visible = false;
        }
        _ => {
            primary_cursor_options.grab_mode = CursorGrabMode::None;
            primary_cursor_options.visible = true;
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    toggle_grab_cursor(primary_cursor_options);
}


fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(primary_cursor_options);
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity), With<Player>>,
) {
    let Ok((transform, mut velocity)) = query.single_mut() else {
        return;
    };

    let mut movement = Vec3::ZERO;

    let forward = transform.forward();
    let right = transform.right();

    if keyboard.pressed(KeyCode::KeyW) {
        movement += *forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement -= *forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement -= *right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement += *right;
    }

    // Ignore camera pitch for movement.
    movement.y = 0.0;

    if movement.length_squared() > 0.0 {
        movement = movement.normalize();
    }

    velocity.linear.x = movement.x * PLAYER_SPEED;
    velocity.linear.z = movement.z * PLAYER_SPEED;

    // Simple grounded check.
    if keyboard.just_pressed(KeyCode::Space) && transform.translation.y <= 1.0 {
        velocity.linear.y = JUMP_IMPULSE;
    }
}


#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::default(),
	Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 1.0))),
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.4),
	Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
	KinematicCharacterController {
            ..KinematicCharacterController::default()
        },
        Damping {
            linear_damping: 2.0,
            angular_damping: 100.0,
        },
        children![
            (
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 65.0_f32.to_radians(),
                    ..default()
                }),
            ),
            // Spawn view model camera.
            (
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 65.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ),
            // Spawn the player's right arm.
            (
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ),	    
        ],
    ));
}


fn spawn_lights(mut commands: Commands) {

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


    // commands.spawn(


    // 	(
    //     PointLight {
    //         color: Color::from(tailwind::ROSE_300),
    //         shadow_maps_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(-2.0, 4.0, -0.75),
    //     // The light source illuminates both the world model and the view model.
    //     RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    // ));
}


fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Move the camera with your mouse.\n",
            "Press arrow up to decrease the FOV of the world model.\n",
            "Press arrow down to increase the FOV of the world model."
        )));
}

fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        // Note that we are not multiplying by delta_time here.
        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
        // This means that if we multiply by delta_time, we will get a smaller rotation than intended by the user.
        // This situation is reversed when reading e.g. analog input from a gamepad however, where the same rules
        // as for keyboard input apply. Such an input should be multiplied by delta_time to get the intended rotation
        // independent of the framerate.
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
        // When the user wants to move the camera back to the horizon, which way should the camera face?
        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
        // so the direction picked will for all intents and purposes be arbitrary.
        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
        // To not run into these issues, we clamp the pitch to a safe range.
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn change_fov(
    input: Res<ButtonInput<KeyCode>>,
    mut world_model_projection: Single<&mut Projection, With<WorldModelCamera>>,
) {
    let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
        unreachable!(
            "The `Projection` component was explicitly built with `Projection::Perspective`"
        );
    };

    if input.pressed(KeyCode::ArrowUp) {
        perspective.fov -= 1.0_f32.to_radians();
        perspective.fov = perspective.fov.max(20.0_f32.to_radians());
    }
    if input.pressed(KeyCode::ArrowDown) {
        perspective.fov += 1.0_f32.to_radians();
        perspective.fov = perspective.fov.min(160.0_f32.to_radians());
    }
}


// use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;
// use bevy::{
//     camera::visibility::RenderLayers, color::palettes::tailwind,
//     input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*,
// };
// use bevy::color::*;

// This line tells the compiler to include the code it finds in src/game.rs
//pub mod game;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
    //.add_plugins(game::game::GamePlugin)
    //.add_systems(Startup, (init_level, spawn_lights))
        // .add_systems(Startup, init_level)
	// .add_systems(Startup, setup)
	// .add_systems(Startup, setup_physics)
	// .add_systems(Update, update_system)
	// .add_systems(Update, read_result_system)
//         .run();
// }


// #[derive(Debug, Component)]
// struct Player;



// fn init_level(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,

// ) {
//      /*
//      * Ground
//      */
//     let ground_size = 100.1;
//     let ground_height = 0.5;

//     commands.spawn((
//         Transform::from_xyz(0.0, -ground_height, 0.0),
//         Collider::cuboid(ground_size, ground_height, ground_size),
//     ));

//     let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
//     let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
//     let material = materials.add(Color::WHITE);

//     // spawn floor
//     commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone()), Transform::from_xyz(0.0, 0.1, 0.0),));

    
// }


// fn spawn_lights(mut commands: Commands) {
//     commands.spawn((
//         PointLight {
//             color: Color::from(tailwind::NEUTRAL_300),
//             shadow_maps_enabled: true,
//             ..default()
//         },
//         Transform::from_xyz(-2.0, 4.0, -0.75),
//         // The light source illuminates both the world model and the view model.
//         //RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
//     ));
// }

// /// For more infomation, see: https://rapier.rs/docs/user_guides/bevy_plugin/character_controller/
// fn setup_physics(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,

// ) {
//     /*
//      * Spawn Player with CharacterController
//      */

//     // commands.spawn((
//     //     //RigidBody::KinematicPositionBased,
//     // 	//Player,
//     // 	RigidBody::Dynamic,
//     // 	GravityScale(0.5),
//     //     Transform::from_xyz(0.0, 4.1, 0.0),
//     // 	Visibility::default(),
//     // 	Collider::cuboid(1.0, 2.0, 1.0),
//     // 	ColliderDebugColor(Srgba::rgb(0.5, 0.5, 0.5).into()),
//     // 	KinematicCharacterController{
//     //         ..KinematicCharacterController::default()
//     //     },
//     // 	Mesh3d(meshes.add(Capsule3d::default())),
//     //     MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
//     // ));

//     commands
//     .spawn(RigidBody::Dynamic)
//     .insert(Transform::from_xyz(0.0, 6.0, 0.0))
//     .insert(GravityScale(1.0))
//     .insert(Sleeping::disabled())
// 	.insert(Ccd::enabled())
// 	.insert(ColliderDebugColor(Srgba::rgb(0.5, 0.5, 0.5).into()))
// 	.insert(Mesh3d(meshes.add(Capsule3d::default())))
// 	.insert(MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))))
// .insert(Collider::cuboid(1.0, 2.0, 1.0));

// }

// /* , With<Player>
// */
// fn update_system(time: Res<Time>, mut controllers: Query<&mut KinematicCharacterController>) {
//     for mut controller in controllers.iter_mut() {
//         //controller.translation = Some(Vec3::new(0.0, -1.0, 0.0) * time.delta_secs());
//     }
// }

// fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
//     for (entity, output) in controllers.iter() {
//         println!(
//             "Entity {:?} moved by {:?} and touches the ground: {:?}",
//             entity, output.effective_translation, output.grounded
//         );
//     }
// }
// #[derive(Component)]
// struct Ground;

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // plane
//     commands.spawn((
//         Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
//         MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
//         Ground,
//     ));

//     // light
//     commands.spawn((
//         DirectionalLight::default(),
//         Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
//     ));

//     // camera
//     commands.spawn((
//         Camera3d::default(),
//         Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
//     ));
// }

// fn character_movement(
//     keys: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//      mut controllers: Query<&mut KinematicCharacterController>,
// ) {
//     for mut controller in controllers.iter_mut() {
// 	let mut movement = Vec3::ZERO;
//         let forward = Vec3::new(1.0, 0., 0.0);
//         let right = Vec3::new(0.0, 0., 1.0);


        

// 	if keys.pressed(KeyCode::KeyW){
//             movement = forward;
// 	}
// 	if keys.pressed(KeyCode::KeyA){

//             movement = right;
// 	}
// 	if keys.pressed(KeyCode::KeyS){
//             movement = -forward;
	    
// 	}
// 	if keys.pressed(KeyCode::KeyD){
//             movement = -right;
// 	}

// 	movement = movement.normalize_or_zero();
// 	controller.translation = Some(
// 	    movement * time.delta_secs()
// 	);

	
// 	controller.translation = Some(Vec3::new(0.0, -0.1, 0.0) * time.delta_secs());
//     }
// }



// fn jump() {
    
// }


