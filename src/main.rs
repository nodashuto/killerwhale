//mod first_person_character;

use std::f32::consts::FRAC_PI_2;

use bevy::{camera::visibility::RenderLayers, color::palettes::tailwind, light::NotShadowCaster};

use bevy::window::PresentMode;
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    text::FontSmoothing,
};

use bevy::window::{WindowMode, WindowResolution}; // for window size


// use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::{
    AccumulatedMouseMotion,
    // AccumulatedMouseScroll
};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_rapier3d::prelude::*;

// use light_consts::lux::AMBIENT_DAYLIGHT;

//use crate::shootingtarget;
pub mod shootingtarget;
//pub mod world;


mod player;
mod world;
mod weapon;
mod hud;

use player::PlayerPlugin;
use world::WorldPlugin;
use weapon::WeaponPlugin;
use hud::HudPlugin;

//use first_person_character::FirstPersonCharacterPlugin;

// use crate::shootingtarget::*;

// use shootingtarget::ShootingTargetPlugin;

const PLAYER_SPEED: f32 = 8.0;
const PLAYER_SPRINTING_SPEED_OLD: f32 = 16.0;
// const PLAYER_JUMP_SPEED: f32 = 3.0;
const PLAYER_GRAVITY: f32 = 30.0;
// const MOUSE_SENSITIVITY: f32 = 0.002;

use bevy_egui::{
    // egui, EguiContexts, 
    EguiPlugin,
    // EguiPrimaryContextPass
};

struct OverlayColor;

impl OverlayColor {
    //const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Basic FPS".into(),
                present_mode: PresentMode::AutoNoVsync,
		//resolution: WindowResolution::new(1920, 1080),
		//mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.init_state::<GameState>()
        .init_resource::<SoundEffect>() // for sound effect
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: FontSize::Px(22.0),
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                // We can also change color of the overlay
                text_color: OverlayColor::GREEN,
                // We can also set the refresh interval for the FPS counter
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    // The minimum acceptable fps
                    min_fps: 30.0,
                    // The target fps
                    target_fps: 250.0, // 144.0
                },
            },
        })
        // .add_plugins(EguiPlugin::default())
        
        // .add_systems(
        //     Startup,
        //     (
        //         spawn_view_model,
        //         //spawn_lights,
        //         spawn_text,
        //     ),
        // )
        .add_systems(Startup, initial_grab_cursor)
        
        // .add_systems(Startup, spawn_crosshair)
        
        // .add_systems(Update, toggle_and_animate_crosshair)
        // .add_systems(Update, (move_player, change_fov, ads_zoom))
        // .add_systems(Update, (update_view_arm, update_view_weapon))
        // .add_systems(Update, fire_weapon)
        
        // .add_systems(
        //     Update,
        //     (
        //         player_movement,
        //         update_grounded,
        //         //jump_start, player_gravity
        //     ),
        // )
        .add_systems(Update, cursor_grab)
        // // .add_systems(Update, check_goal.run_if(in_state(GameState::Playing)))
        // //.add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}


// // example system for Egui
// fn ui_example_system(mut contexts: EguiContexts) -> Result {
//     egui::Window::new("Egui")
//         .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 10.0))
//         .show(contexts.ctx_mut()?, |ui| {
//             ui.label("ASSETS AND GAMEPLAY CURRENTLY IN DEVELOPMENT ");
//         });
//     Ok(())
// }

#[derive(Component)]
struct Crosshair;

#[derive(Component)]
enum CrosshairArm {
    Top,
    Bottom,
    Left,
    Right,
}

const ARM_LENGTH: f32 = 8.0;
const THICKNESS: f32 = 2.0;
const OPEN_GAP: f32 = 80.0;
const CLOSED_GAP: f32 = 2.0;
const CROSSHAIR_SPEED: f32 = 250.0;

pub fn spawn_crosshair(window_query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    let window = window_query.single().unwrap();

    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;

    commands
        .spawn((
            Crosshair,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(center_x),
                top: Val::Px(center_y),
                ..default()
            },
        ))
        .with_children(|parent| {
            let color = BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.6));

            parent.spawn((
                CrosshairArm::Top,
                Node {
                    width: Val::Px(THICKNESS),
                    height: Val::Px(ARM_LENGTH),
                    position_type: PositionType::Absolute,
                    left: Val::Px(-THICKNESS / 2.0),
                    top: Val::Px(-(OPEN_GAP / 2.0 + ARM_LENGTH)),
                    ..default()
                },
                color.clone(),
            ));

            parent.spawn((
                CrosshairArm::Bottom,
                Node {
                    width: Val::Px(THICKNESS),
                    height: Val::Px(ARM_LENGTH),
                    position_type: PositionType::Absolute,
                    left: Val::Px(-THICKNESS / 2.0),
                    top: Val::Px(OPEN_GAP / 2.0),
                    ..default()
                },
                color.clone(),
            ));

            parent.spawn((
                CrosshairArm::Left,
                Node {
                    width: Val::Px(ARM_LENGTH),
                    height: Val::Px(THICKNESS),
                    position_type: PositionType::Absolute,
                    left: Val::Px(-(OPEN_GAP / 2.0 + ARM_LENGTH)),
                    top: Val::Px(-THICKNESS / 2.0),
                    ..default()
                },
                color.clone(),
            ));

            parent.spawn((
                CrosshairArm::Right,
                Node {
                    width: Val::Px(ARM_LENGTH),
                    height: Val::Px(THICKNESS),
                    position_type: PositionType::Absolute,
                    left: Val::Px(OPEN_GAP / 2.0),
                    top: Val::Px(-THICKNESS / 2.0),
                    ..default()
                },
                color,
            ));
        });
}

fn toggle_and_animate_crosshair(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut crosshair_query: Query<&mut Visibility, With<Crosshair>>,
    mut arm_query: Query<(&CrosshairArm, &mut Node)>,
) {
    let _left_pressed = mouse.pressed(MouseButton::Left);
    let right_pressed = mouse.pressed(MouseButton::Right);

    let target_gap = if right_pressed {
        CLOSED_GAP
    } else {
        OPEN_GAP
    };

    let amount = CROSSHAIR_SPEED * time.delta_secs();

    let mut reached_target = true;

    for (arm, mut node) in &mut arm_query {
        match arm {
            CrosshairArm::Top => {
                let target = -(target_gap / 2.0 + ARM_LENGTH);

                if let Val::Px(current) = node.top {
                    let new_value = move_towards(current, target, amount);
                    node.top = Val::Px(new_value);

                    if (new_value - target).abs() > 0.01 {
                        reached_target = false;
                    }
                }
            }

            CrosshairArm::Bottom => {
                let target = target_gap / 2.0;

                if let Val::Px(current) = node.top {
                    let new_value = move_towards(current, target, amount);
                    node.top = Val::Px(new_value);

                    if (new_value - target).abs() > 0.01 {
                        reached_target = false;
                    }
                }
            }

            CrosshairArm::Left => {
                let target = -(target_gap / 2.0 + ARM_LENGTH);

                if let Val::Px(current) = node.left {
                    let new_value = move_towards(current, target, amount);
                    node.left = Val::Px(new_value);

                    if (new_value - target).abs() > 0.01 {
                        reached_target = false;
                    }
                }
            }

            CrosshairArm::Right => {
                let target = target_gap / 2.0;

                if let Val::Px(current) = node.left {
                    let new_value = move_towards(current, target, amount);
                    node.left = Val::Px(new_value);

                    if (new_value - target).abs() > 0.01 {
                        reached_target = false;
                    }
                }
            }
        }
    }

    for mut visibility in &mut crosshair_query {
        if right_pressed && reached_target {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

fn _animate_crosshair(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&CrosshairArm, &mut Node)>,
) {
    let target_gap = if mouse.pressed(MouseButton::Right) {
        CLOSED_GAP
    } else {
        OPEN_GAP
    };

    let amount = CROSSHAIR_SPEED * time.delta_secs();

    for (arm, mut node) in &mut query {
        match arm {
            CrosshairArm::Top => {
                let target = -(target_gap / 2.0 + ARM_LENGTH);

                if let Val::Px(current) = node.top {
                    node.top = Val::Px(move_towards(current, target, amount));
                }
            }
            CrosshairArm::Bottom => {
                let target = target_gap / 2.0;

                if let Val::Px(current) = node.top {
                    node.top = Val::Px(move_towards(current, target, amount));
                }
            }
            CrosshairArm::Left => {
                let target = -(target_gap / 2.0 + ARM_LENGTH);

                if let Val::Px(current) = node.left {
                    node.left = Val::Px(move_towards(current, target, amount));
                }
            }
            CrosshairArm::Right => {
                let target = target_gap / 2.0;

                if let Val::Px(current) = node.left {
                    node.left = Val::Px(move_towards(current, target, amount));
                }
            }
        }
    }
}

fn _toggle_crosshair(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Visibility, With<Crosshair>>,
) {
    for mut visibility in &mut query {
        *visibility = if mouse.pressed(MouseButton::Right) {
            Visibility::Visible
        } else {
            Visibility::Visible
        };
    }
}

fn move_towards(current: f32, target: f32, amount: f32) -> f32 {
    if (target - current).abs() <= amount {
        target
    } else {
        current + (target - current).signum() * amount
    }
}

#[derive(Component)]
struct Player;


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
            handle: asset_server.load("sounds/glock_single_shot_modifiy.ogg"),
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


fn check_jump(player: &mut PlayerController, keyboard: &ButtonInput<KeyCode>) {
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
        PLAYER_SPRINTING_SPEED_OLD
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


#[derive(Component)]
pub struct PlayerController {
    pub velocity: Vec3,
    pub isgrounded: bool,
    // Time remaining in the jump/landing penalty.
    jump_penalty_time: f32,
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
            // Time remaining in the jump/landing penalty.
            jump_penalty_time: 0.0,
        }
    }
}


fn update_grounded(mut query: Query<(&mut PlayerController, &KinematicCharacterControllerOutput)>) {
    for (mut player, output) in query.iter_mut() {
        player.isgrounded = output.grounded;
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
            Vec2::new(0.001, 0.001),
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
    //let gun_mesh = asset_server.load("models/gun-model-0004.glb#Mesh0/Primitive0");
    let gun_mesh = asset_server.load("models/20260812-glock17-viewmodel.glb#Mesh0/Primitive0");
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
        Transform::from_xyz(0.0, 20.0, 0.0),
        Visibility::default(),
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 1.0))),
        //RigidBody::Dynamic,
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(1.828, 0.3),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            autostep: Some(CharacterAutostep {
                // Autostep if the step height is smaller than 0.1, and its width larger than 0.2.
                max_height: CharacterLength::Absolute(0.1),
                min_width: CharacterLength::Absolute(0.2),
                include_dynamic_bodies: true,
            }),
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
		Transform::from_xyz(0.0, 0.0, 0.0),
                Projection::from(PerspectiveProjection {
                    fov: 65.0_f32.to_radians(),
                    ..default()
                }),
                // DistanceFog {
                //     color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                //     directional_light_color: Color::srgba(1.0, 1.0, 1.0, 1.0),
                //     directional_light_exponent: 30.0,
                //     falloff: FogFalloff::from_visibility_colors(
                //         15.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                //         Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                //         Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
                //     ),
                // },
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
                    //translation: Vec3::new(0.8, -0.8, -1.2),
		    translation: Vec3::new(0.0, 0.0, 0.0),
                    rotation: Quat::from_rotation_y(std::f32::consts::PI),
                    //scale: Vec3::new(0.1, 0.1, 0.1),
		    scale: Vec3::new(0.6, 0.6, 0.6),
                },
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                //NotShadowCaster,
            ),
        ],
    ));
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
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.8;
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
// const WEAPON_IDLE: Vec3 = Vec3::new(0.8, -0.9, -1.25);
// const WEAPON_AIM: Vec3 = Vec3::new(0.0, -0.76, -0.4);

const WEAPON_IDLE: Vec3 = Vec3::new(2.0, -0.3, -3.0);
const WEAPON_AIM: Vec3 = Vec3::new(0.0, 0.7, -2.0);



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

    let max_distance = 30.0;

    // Exclude player's hitbox when ray casting
    let player_entity = player_query.single().unwrap();
    let filter = QueryFilter::default().exclude_rigid_body(player_entity);

    if let Ok(ctx) = rapier_context.single() {
        if let Some((entity, toi)) = ctx.cast_ray(origin, *direction, max_distance, true, filter) {
            println!("Hit {:?} at {}", entity, toi);

            if let Ok(mut health) = health_query.get_mut(entity) {
                health.current -= 50.0;
                println!("Remaining HP: {}", health.current);
            }

            //let hit_position = origin + *direction * toi;
            //println!("Impact position: {:?}", hit_position);
        }
    }
}
