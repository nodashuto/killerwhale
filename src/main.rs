use std::f32::consts::FRAC_PI_2;

use bevy::{camera::visibility::RenderLayers, color::palettes::tailwind, light::NotShadowCaster};

use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_rapier3d::prelude::*;

use light_consts::lux::AMBIENT_DAYLIGHT;

//use crate::shootingtarget;
pub mod shootingtarget;
pub mod world;

use crate::shootingtarget::*;

use shootingtarget::ShootingTargetPlugin;

const PLAYER_SPEED: f32 = 3.0;
const PLAYER_JUMP_SPEED: f32 = 3.0;
const PLAYER_GRAVITY: f32 = 40.0;
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
        .init_state::<GameState>()
        .init_resource::<SoundEffect>() // for sound effect
        .add_plugins(world::WorldPlugin)
        //.add_plugins(shootingtarget::ShootingTargetPlugin)
        //.add_plugins(RapierDebugRenderPlugin::default()) // Uncomment for collider visualization
        //.insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.15)))
        //.add_systems(Startup, setup)
        // .add_systems(Startup, initial_grab_cursor)
        // .add_systems(Startup, initial_grab_on_player_spawn)
        // .add_systems(Update, player_look)
        // .add_systems(Update, player_movement)
        //.add_systems(Startup, setup) //needthis
        //.add_systems(Update, mouse_look)
        // .add_systems(Update, grab_mouse)
        .add_systems(Startup, (
	    spawn_view_model,
	    //spawn_lights,
	    spawn_text))
        .add_systems(Startup, initial_grab_cursor)
        .add_systems(Startup, setup_goal)
        .add_systems(Update, (move_player, change_fov, ads_zoom))
        .add_systems(Update, (update_view_arm, update_view_weapon))
        .add_systems(Update, fire_weapon)
        //.add_systems(Update,player_movement) //needthis
        .add_systems(
            Update,
            (
                player_movement,
                update_grounded,
                //jump_start, player_gravity
            ),
        )
        .add_systems(Update, cursor_grab)
        .add_systems(Update, check_goal.run_if(in_state(GameState::Playing)))
        .run();
}

#[derive(Component)]
struct Player;

// #[derive(Component)]
// struct PlayerCamera;

// #[derive(Component)]
// struct LookAngles {
//     yaw: f32,
//     pitch: f32,
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Light
//     commands.spawn((
//         PointLight {
//             intensity: 5000.0,
//             shadow_maps_enabled: true,
//             ..default()
//         },
//         Transform::from_xyz(5.0, 10.0, 5.0),
//     ));

//     // Floor
//     commands.spawn((
//         Mesh3d(meshes.add(Cuboid::new(50.0, 1.0, 50.0))),
//         MeshMaterial3d(materials.add(Color::srgb(0.3, 0.6, 0.3))),
//         Transform::from_xyz(0.0, -0.5, 0.0),
//         RigidBody::Fixed,
//         Collider::cuboid(25.0, 0.5, 25.0),
//     ));

//     // Red box
//     commands.spawn((
//         Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
//         MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
//         Transform::from_xyz(9.0, -0.3, 6.0),
//         RigidBody::Fixed,
//         Collider::cuboid(0.5, 0.5, 0.5),
//     ));

//     // second Red box
//     commands.spawn((
//         Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
//         MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
//         Transform::from_xyz(9.0, 0.1, 5.0),
//         RigidBody::Fixed,
//         Collider::cuboid(0.5, 0.5, 0.5),
//     ));

//     // Blue Wall
//     commands.spawn((
//         Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, 1.0))),
//         MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
//         Transform::from_xyz(-10.0, 0.0, -10.0),
//         RigidBody::Fixed,
//         Collider::cuboid(2.5, 2.5, 0.5),
//     ));

//     // Player
//     commands.spawn((
//         Player,
//         Mesh3d(meshes.add(Capsule3d::default())),
//         MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 1.0))),
//         Transform::from_xyz(0.0, 2.0, 0.0),
//         RigidBody::Dynamic,
//         Collider::capsule_y(0.5, 0.4),
//         Velocity::default(),
//         LockedAxes::ROTATION_LOCKED,
//         GravityScale(1.0),
// 	KinematicCharacterController {
//             ..KinematicCharacterController::default()
//         },
//         Damping {
//             linear_damping: 2.0,
//             angular_damping: 100.0,
//         },
//         Camera3d::default(),
//     ));
// }

#[derive(Resource, Deref)]
struct SoundEffect {
    handle: Handle<AudioSource>,
}

// We can setup the logic for how to load our assets in the `FromWorld` trait.
// This code is called via `init_resource`.
impl FromWorld for SoundEffect {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        SoundEffect {
            handle: asset_server.load("sounds/glock_single_shot_modify.ogg"),
        }
    }
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

// Ground acceleration.
// Higher = reaches max speed faster.
const GROUND_ACCEL: f32 = 20.0;
// Air acceleration.
// Lower than ground acceleration gives you reduced air control.
const AIR_ACCEL: f32 = 6.0;
// Jump height in world units.
const JUMP_HEIGHT: f32 = 0.9;

fn accelerate(velocity: &mut Vec3, wish_dir: Vec3, wish_speed: f32, acceleration: f32, dt: f32) {
    if wish_dir == Vec3::ZERO || wish_speed <= 0.0 {
        return;
    }
    // Velocity in the direction the player wants to move.
    let current_speed = velocity.dot(wish_dir);
    // How much more speed we need.
    let add_speed = wish_speed - current_speed;
    if add_speed <= 0.0 {
        return;
    }
    // Amount of acceleration this frame.
    let accel_speed = acceleration * dt * wish_speed;
    let accel_speed = accel_speed.min(add_speed);
    *velocity += wish_dir * accel_speed;
}

fn friction(velocity: &mut Vec3, friction: f32, stop_speed: f32, dt: f32) {
    let speed = Vec2::new(velocity.x, velocity.z).length();

    if speed < 0.001 {
        velocity.x = 0.0;
        velocity.z = 0.0;
        return;
    }

    let control = speed.max(stop_speed);
    let drop = control * friction * dt;

    let new_speed = (speed - drop).max(0.0);
    let scale = new_speed / speed;

    velocity.x *= scale;
    velocity.z *= scale;
}

fn walk_move(velocity: &mut Vec3, wish_dir: Vec3, wish_speed: f32, dt: f32) {
    friction(velocity, 15.0, 3.0, dt);
    // Ground movement accelerates quickly toward the desired speed.
    accelerate(velocity, wish_dir, wish_speed, GROUND_ACCEL, dt);
    // Ground movement should not accumulate vertical velocity.
    //
    // We only remove downward velocity here. This prevents a small
    // downward velocity from making the player fall through the floor.
    if velocity.y < 0.0 {
        velocity.y = 0.0;
    }
}

fn air_move(velocity: &mut Vec3, wish_dir: Vec3, wish_speed: f32, dt: f32) {
    // Air acceleration is deliberately weaker than ground acceleration.
    accelerate(velocity, wish_dir, wish_speed, AIR_ACCEL, dt);
}

fn check_jump(player: &mut PlayerController, keyboard: &ButtonInput<KeyCode>) {
    if !player.isgrounded {
        return;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        // v² = 2gh // // This is the same calculation used by the Jump_Start() function.
        player.velocity.y = (2.0 * PLAYER_GRAVITY * JUMP_HEIGHT).sqrt();
        player.isgrounded = false;
    }
}

// fn walk_move()
// {}

// fn air_move(){

// }

// fn check_jump(){

// }

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &mut KinematicCharacterController,
        &mut PlayerController,
    )>,
) {
    let dt = time.delta_secs();
    for (transform, mut controller, mut player) in query.iter_mut() {
        let mut input = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            input.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            input.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            input.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            input.x += 1.0;
        }
        if input.length_squared() > 0.0 {
            input = input.normalize();
        }
        let mut wish_dir = transform.rotation * input;
        wish_dir.y = 0.0;
        if wish_dir.length_squared() > 0.0 {
            wish_dir = wish_dir.normalize();
        }
        let wish_speed = if wish_dir != Vec3::ZERO {
            PLAYER_SPEED
        } else {
            0.0
        };
        check_jump(&mut player, &keyboard);
        if player.isgrounded {
            walk_move(&mut player.velocity, wish_dir, wish_speed, dt);
        } else {
            air_move(&mut player.velocity, wish_dir, wish_speed, dt);
            player.velocity.y -= PLAYER_GRAVITY * dt;
        }
        controller.translation = Some(player.velocity * dt);
    }
}

// fn player_movement(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut query: Query<(&Transform, &mut KinematicCharacterController)>,
//     mut player_controller: Query<&mut PlayerController>,
// ) {
//     let mut input = Vec3::ZERO;
//     let air_friction = 0.01;

//     for player in player_controller.iter_mut() {
//         // WASD when grounded
//         if player.isgrounded {
//             if keyboard.pressed(KeyCode::KeyW) {
//                 input.z -= 1.0;
//             }
//             if keyboard.pressed(KeyCode::KeyS) {
//                 input.z += 1.0;
//             }
//             if keyboard.pressed(KeyCode::KeyA) {
//                 input.x -= 1.0;
//             }
//             if keyboard.pressed(KeyCode::KeyD) {
//                 input.x += 1.0;
//             }
//         } else {
//             if keyboard.pressed(KeyCode::KeyW) {
//                 input.z -= 1.0 * air_friction;
//             }
//             if keyboard.pressed(KeyCode::KeyS) {
//                 input.z += 1.0 * air_friction;
//             }
//             if keyboard.pressed(KeyCode::KeyA) {
//                 input.x -= 1.0 * air_friction;
//             }
//             if keyboard.pressed(KeyCode::KeyD) {
//                 input.x += 1.0 * air_friction;
//             }
//         }
//     }

//     if input == Vec3::ZERO {
//         return;
//     }

//     let speed = 5.0;
//     let dt = time.delta_secs();

//     for (transform, mut controller) in query.iter_mut() {
//         // Convert local WASD direction into world direction
//         let mut direction = transform.rotation * input;

//         // Keep movement horizontal
//         //direction.y = 0.0;

//         for player in player_controller.iter_mut() {
//             // WASD when grounded
//             if player.isgrounded {
//                 // Keep horizontal movement when grounded
//                 direction.y = 0.0;
//             } else {
//                 // Add down pull when not grounded

//                 direction.y = -PLAYER_GRAVITY * 00.1 * dt;
//             }
//         }

//         if direction != Vec3::ZERO {
//             direction = direction.normalize();
//         }

//         controller.translation = Some(direction * PLAYER_SPEED * dt);
//     }
// }

#[derive(Component)]
pub struct PlayerController {
    pub velocity: Vec3,
    pub isgrounded: bool,
}

// #[derive(Component)]
// struct PlayerController {
//     vertical_velocity: f32,
//     isgrounded: bool,
// }

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            //vertical_velocity: 0.0,
            isgrounded: false,
        }
    }
}

fn calculate_jump_speed(height: f32, time_to_peak: f32) -> f32 {
    (1.0 * height) / time_to_peak
}

fn jump_start(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerController>,
    time: Res<Time>,
    mut controllers: Query<(&Transform, &mut KinematicCharacterController)>,
) {
    let dt = time.delta_secs();

    let jump_height = 2.0;
    let jump_time_to_peak = 500.0 * dt;

    //let jump_strength =  calculate_jump_speed(jump_height, jump_time_to_peak);

    let jump_strength = 5.0;

    let test_jump_height = 10.0f32; // 39.0f32
    let test_gravity = 1.0f32;
    let mut jump_velocity_squared = (test_jump_height + test_jump_height) * test_gravity;

    /* jump velocity is determined by factor. factor can be determined by player-state
     *
     */
    let factor = 1.0f32;

    // Modify jump strength after certain landing states.
    jump_velocity_squared = jump_velocity_squared / factor;

    for (transform, mut controller) in controllers.iter_mut() {
        for mut player in query.iter_mut() {
            if keyboard.just_pressed(KeyCode::Space) && player.isgrounded {
                player.velocity.y = jump_velocity_squared.sqrt();
                controller.translation = Some(Vec3::new(0.0, player.velocity.y, 0.0) * dt);
                println!("Player Jump");
                player.isgrounded = false;
            }
        }
    }
}

fn update_grounded(mut query: Query<(&mut PlayerController, &KinematicCharacterControllerOutput)>) {
    for (mut player, output) in query.iter_mut() {
        player.isgrounded = output.grounded;
    }
}

fn player_gravity(
    time: Res<Time>,
    mut query: Query<(&mut PlayerController, &mut KinematicCharacterController)>,
) {
    // let gravity = -10.0;
    // let dt = time.delta_secs();

    // for (mut player, mut controller) in query.iter_mut() {
    //     if !player.isgrounded {
    //         player.vertical_velocity += gravity * dt;

    //         controller.translation = Some(Vec3::new(0.0, player.vertical_velocity * dt, 0.0));
    //     } else if player.vertical_velocity < 0.0 {
    //         // Prevent velocity from building up while standing.
    //         player.vertical_velocity = 0.0;
    //     }
    // }
}

// fn player_movement(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut query: Query<(&Transform, &mut Velocity), With<Player>>,
// ) {
//     let Ok((transform, mut velocity)) = query.single_mut() else {
//         return;
//     };

//     let mut movement = Vec3::ZERO;

//     let forward = transform.forward();
//     let right = transform.right();

//     if keyboard.pressed(KeyCode::KeyW) {
//         movement += *forward;
//     }
//     if keyboard.pressed(KeyCode::KeyS) {
//         movement -= *forward;
//     }
//     if keyboard.pressed(KeyCode::KeyA) {
//         movement -= *right;
//     }
//     if keyboard.pressed(KeyCode::KeyD) {
//         movement += *right;
//     }

//     // Ignore camera pitch for movement.
//     movement.y = 0.0;

//     if movement.length_squared() > 0.0 {
//         movement = movement.normalize();
//     }

//     velocity.linear.x = movement.x * PLAYER_SPEED;
//     velocity.linear.z = movement.z * PLAYER_SPEED;

//     // Simple grounded check.
//     if keyboard.just_pressed(KeyCode::Space) && transform.translation.y <= 1.0 {
//         velocity.linear.y = JUMP_IMPULSE;
//     }
// }

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
    asset_server: Res<AssetServer>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    // Load the mesh from the GLB
    let gun_mesh = asset_server.load("models/gun-model-0004.glb#Mesh0/Primitive0");
    let gun_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        metallic: 0.1,
        ..default()
    });

    commands.spawn((
        Player,
        PlayerController {
            ..PlayerController::default()
        },
        CameraSensitivity::default(),
        Transform::from_xyz(0.0, 2.0, 0.0),
        Visibility::default(),
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 1.0))),
        //RigidBody::Dynamic,
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.5, 0.4),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            ..default()
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
                ViewArm, // component marker for ADS translation
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ),
            //Spawn the player's gun model
            (
                ViewWeapon,
                Mesh3d(gun_mesh),
                MeshMaterial3d(gun_material),
                //transform::from_xyz(0.2, -0.1, -0.25),
                Transform {
                    translation: Vec3::new(0.8, -0.8, -1.2),
                    rotation: Quat::from_rotation_y(std::f32::consts::PI),
                    scale: Vec3::new(0.1, 0.1, 0.1),
                },
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                //NotShadowCaster,
            ),
        ],
    ));
}

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
            color: Color::from(tailwind::NEUTRAL_200),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 2.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

// /// original spawn text here
// fn spawn_text(mut commands: Commands) {
//     commands
//         .spawn(Node {
//             position_type: PositionType::Absolute,
//             bottom: px(12),
//             left: px(12),
//             ..default()
//         })
//         .with_child(Text::new(concat!(
//             "Move the camera with your mouse.\n",
//             "Press arrow up to decrease the FOV of the world model.\n",
//             "Press arrow down to increase the FOV of the world model."
//         )));
// }

fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Bevy = 0.19.0\n",
            "Apple shooter\n",
            " "
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
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 1.0;
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

// added myself
// fn toggle_ADS(
//     buttons: Res<ButtonInput<MouseButton>>,
//     input: Res<ButtonInput<KeyCode>>,
//     mut world_model_projection: Single<&mut Projection, With<WorldModelCamera>>,
// ) {
//     if !buttons.just_pressed(MouseButton::Right) {
//         return;
//     }

//     let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
//         unreachable!(
//             "The `Projection` component was explicitly built with `Projection::Perspective`"
//         );
//     };

//     if input.pressed(KeyCode::ArrowUp) {
//         perspective.fov -= 1.0_f32.to_radians();
//         perspective.fov = perspective.fov.max(20.0_f32.to_radians());
//     }
//     if input.pressed(KeyCode::ArrowDown) {
//         perspective.fov += 1.0_f32.to_radians();
//         perspective.fov = perspective.fov.min(160.0_f32.to_radians());
//     }

// }

/// Hold right mouse button to zoom in with WorldModelCamera.
fn ads_zoom(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut world_model_projection: Single<&mut Projection, With<WorldModelCamera>>,
) {
    let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
        unreachable!();
    };

    let target = if buttons.pressed(MouseButton::Right) {
        45.0_f32.to_radians()
    } else {
        65.0_f32.to_radians()
    };

    let speed = 10.0;
    perspective.fov += (target - perspective.fov) * speed * time.delta_secs();
}

#[derive(Component)]
struct ViewArm;
const ARM_IDLE: Vec3 = Vec3::new(0.4, -0.35, -0.45);
const ARM_AIM: Vec3 = Vec3::new(0.0, -0.40, -0.55);

fn update_view_arm(
    buttons: Res<ButtonInput<MouseButton>>,
    mut arm: Query<&mut Transform, With<ViewArm>>,
    time: Res<Time>,
) {
    let target = if buttons.pressed(MouseButton::Right) {
        ARM_AIM
    } else {
        ARM_IDLE
    };

    let mut transform = arm.single_mut().unwrap();

    // Smooth movement
    let speed = 32.0;
    transform.translation = transform
        .translation
        .lerp(target, speed * time.delta_secs());
}

#[derive(Component)]
struct ViewWeapon;
const WEAPON_IDLE: Vec3 = Vec3::new(0.8, -0.9, -1.25);
const WEAPON_AIM: Vec3 = Vec3::new(0.0, -0.76, -0.4);

fn update_view_weapon(
    buttons: Res<ButtonInput<MouseButton>>,
    mut weapon: Query<&mut Transform, With<ViewWeapon>>,
    time: Res<Time>,
) {
    let target = if buttons.pressed(MouseButton::Right) {
        WEAPON_AIM
    } else {
        WEAPON_IDLE
    };

    let mut transform = weapon.single_mut().unwrap();

    // Smooth movement
    let speed = 32.0;
    transform.translation = transform
        .translation
        .lerp(target, speed * time.delta_secs());
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

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Playing,
    Results,
}

#[derive(Component)]
struct Goal {
    radius: f32,
}
fn check_goal(
    player: Query<&Transform, With<Player>>,
    goals: Query<(&Transform, &Goal)>,
    //mut next_state: ResMut<NextState<GameState>>,
) {
    let player_pos = player.single().unwrap().translation;
    //let goal_pos = goals.single();

    for (goal_transform, goal) in &goals {
        let distance = player_pos.distance(goal_transform.translation);

        if distance <= goal.radius {
            println!("Reached the goal!");
            // next_state.set(GameState::Results);
        }
    }
}

fn setup_goal(mut commands: Commands) {
    // Spawn goal
    commands.spawn((
        Goal { radius: 2.0 },
        Transform::from_xyz(20.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

fn fire_weapon(
    buttons: Res<ButtonInput<MouseButton>>,
    camera: Query<&GlobalTransform, With<WorldModelCamera>>,
    rapier_context: ReadRapierContext,
    mut health_query: Query<&mut Health>,
    player_query: Query<Entity, With<Player>>,
    sound_effect: Res<SoundEffect>,
    mut commands: Commands,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    println!("Shots fired");
    // play sound effect
    commands.spawn((
        AudioPlayer::new(sound_effect.clone()),
        PlaybackSettings::DESPAWN,
    ));

    let transform = camera.single().unwrap();

    let origin = transform.translation();
    let direction = transform.forward();

    let max_distance = 100.0;

    // Exclude player's hitbox when ray casting
    let player_entity = player_query.single().unwrap();
    let filter = QueryFilter::default().exclude_rigid_body(player_entity);

    if let Ok(ctx) = rapier_context.single() {
        if let Some((entity, toi)) = ctx.cast_ray(origin, *direction, max_distance, true, filter) {
            //println!("Hit {:?} at {}", entity, toi);

            if let Ok(mut health) = health_query.get_mut(entity) {
                health.current -= 50.0;
                println!("Remaining HP: {}", health.current);
            }

            //let hit_position = origin + *direction * toi;
            //println!("Impact position: {:?}", hit_position);
        }
    }
}
