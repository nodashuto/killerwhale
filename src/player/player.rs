use std::f32::consts::FRAC_PI_2;

use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*,
};
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Player systems
        app.add_systems(Startup, player_plugin_loaded);
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (update_grounded, player_look, player_movement));
    }
}

fn player_plugin_loaded() {
    println!("player plugin is loaded");
}

#[derive(Component)]
struct Player;


const MOUSE_SENSITIVITY: f32 = 0.001;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Ground,
    Air,
    Sliding,
    WallRunning,
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

#[derive(Component)]
struct Head {
    pitch: f32,
}

impl Default for Head {
    fn default() -> Self {
        Self { pitch: 0.0 }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
    // let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    // let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    // // Player body
    // let player_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    // let player_material = materials.add(Color::srgb(0.2, 0.7, 1.0));

    commands
        .spawn((
            Player,
            PlayerPhysicsController {
                ..PlayerPhysicsController::default()
            },
            Transform::from_xyz(0.0, 1.0, 10.0),
            Visibility::default(),
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.9, 0.3),
            LockedAxes::ROTATION_LOCKED,
            GravityScale(1.0),
            KinematicCharacterController {
                offset: CharacterLength::Absolute(0.01),
                autostep: Some(CharacterAutostep {
                    // Autostep if the step height is smaller than 0.1, and its width larger than 0.2.
                    max_height: CharacterLength::Absolute(0.1),
                    min_width: CharacterLength::Absolute(0.005),
                    include_dynamic_bodies: true,
                }),
                ..default()
            },
            Damping {
                linear_damping: 2.0,
                angular_damping: 100.0,
            },
        ))
        .with_children(|player| {
            player
                .spawn((
                    Head::default(),
                    Transform::from_xyz(0.0, 1.70, 0.0),
                    Visibility::default(),
                ))
                .with_children(|head| {
                    head.spawn((PlayerCamera, Camera3d::default(), Transform::default()));

                    // head.spawn((
                    //     Camera3d::default(),
                    //     RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                    // ));
                });
        });

    // commands.spawn((
    //     Player,
    //     // PlayerLook::default(),

    //     // Player position and horizontal rotation.
    //     Transform::from_xyz(0.0, 0.0, 10.0),
    //     Visibility::default(),
    //     children![
    //         (
    //             PlayerCamera,
    //             Camera3d::default(),
    //             CameraSensitivity::default(),
    //             Transform::from_xyz(0.0, 2.0, 0.0),
    //         ),
    // // World camera
    // (
    //     WorldModelCamera,
    //     Camera3d::default(),
    //     Projection::from(PerspectiveProjection {
    //         fov: 90.0_f32.to_radians(),
    //         ..default()
    //     }),
    // ),

    // View-model camera
    // (
    //     Camera3d::default(),
    //     Camera {
    //         order: 1,
    //         ..default()
    //     },
    //     Projection::from(PerspectiveProjection {
    //         fov: 70.0_f32.to_radians(),
    //         ..default()
    //     }),
    //     RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
    // ),

    // // Player's arm
    // (
    //     Mesh3d(arm),
    //     MeshMaterial3d(arm_material),
    //     Transform::from_xyz(0.2, -0.1, -0.25),
    //     RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
    //     NotShadowCaster,
    // ),

    // // Optional player body mesh
    // (
    //     Mesh3d(player_mesh),
    //     MeshMaterial3d(player_material),
    // ),
    //     ],
    // ));
}

#[derive(Debug, Component)]
struct PlayerCamera;

fn player_look(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Single<&mut Transform, With<Player>>,
    head: Single<(&mut Transform, &mut Head), Without<Player>>,
) {
    let delta = accumulated_mouse_motion.delta;

    if delta == Vec2::ZERO {
        return;
    }

    let (mut head_transform, mut head) = head.into_inner();

    // Player = yaw
    player.rotate_y(-delta.x * MOUSE_SENSITIVITY);

    // Head = pitch
    head.pitch -= delta.y * MOUSE_SENSITIVITY;

    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    head.pitch = head.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

    head_transform.rotation = Quat::from_rotation_x(head.pitch);
}

#[derive(Component)]
pub struct PlayerPhysicsController {
    pub velocity: Vec3,
    pub isgrounded: bool,
    // Time remaining in the jump/landing penalty.
    jump_penalty_time: f32,
}

impl Default for PlayerPhysicsController {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            //vertical_velocity: 0.0,
            isgrounded: false,
            // Time remaining in the jump/landing penalty.
            jump_penalty_time: 0.0,
        }
    }
}

// fn player_movement(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut query: Query<
//         (
//             &Transform,
//             &mut KinematicCharacterController,
//             &mut PlayerPhysicsController,
//         ),
//         With<Player>,
//     >,
// ) {
//     let dt = time.delta_secs();
//     for (transform, mut controller, mut player) in query.iter_mut() {
//         let mut input = Vec3::ZERO;
//         if keyboard.pressed(KeyCode::KeyW) {
//             input.z -= 1.0;
//         }
//         if keyboard.pressed(KeyCode::KeyS) {
//             input.z += 1.0;
//         }
//         if keyboard.pressed(KeyCode::KeyA) {
//             input.x -= 1.0;
//         }
//         if keyboard.pressed(KeyCode::KeyD) {
//             input.x += 1.0;
//         }
//         if input.length_squared() > 0.0 {
//             input = input.normalize();
//         }
//         let mut wish_dir = transform.rotation * input;
//         wish_dir.y = 0.0;
//         if wish_dir.length_squared() > 0.0 {
//             wish_dir = wish_dir.normalize();
//         }
//         controller.translation = Some(player.velocity * dt);
//     }

// }

// this need in system
fn update_grounded(
    mut query: Query<(
        &mut PlayerPhysicsController,
        &KinematicCharacterControllerOutput,
    )>,
) {
    for (mut player, output) in query.iter_mut() {
        player.isgrounded = output.grounded;
    }
}

const PLAYER_SPEED: f32 = 8.0;
const PLAYER_GRAVITY: f32 = 20.32;
const PLAYER_SPRINTING_SPEED: f32 = 14.0;
// Ground acceleration.
// Higher = reaches max speed faster.
const GROUND_ACCEL: f32 = 18.0;
// Air acceleration.
// Lower than ground acceleration gives you reduced air control.
const AIR_ACCEL: f32 = 10.0;
// Jump height in world units.
const JUMP_HEIGHT: f32 = 1.0;

// Add jump penalty
const JUMP_PENALTY_DURATION: f32 = 0.6;
const JUMP_SLOWDOWN_SPEED: f32 = 0.5;
const JUMP_LAND_SLOWDOWN_TIME: f32 = 1.7;
const JUMP_REJUMP_FACTOR: f32 = 2.5;

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
    friction(velocity, 6.0, 2.0, dt);
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

fn get_jump_land_factor(jump_penalty_time: f32) -> f32 {
    if jump_penalty_time <= 0.0 {
        return 1.0;
    }

    let elapsed = JUMP_PENALTY_DURATION - jump_penalty_time;

    if elapsed >= JUMP_LAND_SLOWDOWN_TIME {
        JUMP_REJUMP_FACTOR
    } else {
        elapsed * 1.5 / JUMP_LAND_SLOWDOWN_TIME + 1.0
    }
}

fn check_jump(player: &mut PlayerPhysicsController, keyboard: &ButtonInput<KeyCode>) {
    if !player.isgrounded {
        return;
    }

    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let normal_velocity = (2.0 * PLAYER_GRAVITY * JUMP_HEIGHT).sqrt();

    let land_factor = get_jump_land_factor(player.jump_penalty_time);

    player.velocity.y = normal_velocity / land_factor.sqrt();

    player.isgrounded = false;
    player.jump_penalty_time = JUMP_PENALTY_DURATION;
}

fn get_move_speed(keyboard: &ButtonInput<KeyCode>) -> f32 {
    // if keyboard.pressed(MouseButton::Left) {
    // 	PLAYER_SPEED
    // }

    if keyboard.pressed(KeyCode::KeyW) && keyboard.pressed(KeyCode::ShiftLeft)
        || keyboard.pressed(KeyCode::ShiftRight)
    {
        PLAYER_SPRINTING_SPEED
    } else {
        PLAYER_SPEED
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &mut KinematicCharacterController,
        &mut PlayerPhysicsController,
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

        // Handle Player Jump
        check_jump(&mut player, &keyboard);

        player.jump_penalty_time = (player.jump_penalty_time - dt).max(0.0);

        // Calculate jump penalty
        let penalty_scale = if player.jump_penalty_time > 0.0 {
            JUMP_SLOWDOWN_SPEED
        } else {
            1.0
        };

        let wish_speed = if wish_dir != Vec3::ZERO {
            get_move_speed(&keyboard) * penalty_scale
        } else {
            0.0
        };
        // let wish_speed = if wish_dir != Vec3::ZERO {
        //     get_move_speed(&keyboard)
        // } else {
        //     0.0
        // };

        if player.isgrounded {
            walk_move(&mut player.velocity, wish_dir, wish_speed, dt);
        } else {
            air_move(&mut player.velocity, wish_dir, wish_speed, dt);
            player.velocity.y -= PLAYER_GRAVITY * dt;
        }
        controller.translation = Some(player.velocity * dt);
    }
}
