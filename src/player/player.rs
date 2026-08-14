use std::f32::consts::FRAC_PI_2;

use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Player systems
        app.add_systems(Startup, player_plugin_loaded);
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (player_look, player_movement));
    }
}

fn player_plugin_loaded() {
    println!("player plugin is loaded");
}

#[derive(Component)]
struct Player;

const MOVE_SPEED: f32 = 5.0;
const MOUSE_SENSITIVITY: f32 = 0.002;

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
        Self {
            pitch: 0.0,
        }
    }
}

fn player_look(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Single<&mut Transform, With<Player>>,
    mut head: Single<(&mut Transform, &mut Head), Without<Player>>,
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

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut input = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        input.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    if input == Vec2::ZERO {
        return;
    }

    // Prevent diagonal movement from being faster.
    let input = input.normalize();

    // Player handles yaw, so its forward/right vectors are
    // exactly what we want for horizontal movement.
    let forward = player.forward();
    let right = player.right();

    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize();
    let right = Vec3::new(right.x, 0.0, right.z).normalize();

    let direction = right * input.x + forward * input.y;

    player.translation += direction * MOVE_SPEED * time.delta_secs();
}

#[derive(Debug, Component)]
struct PlayerCamera;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    // Player body
    let player_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let player_material = materials.add(Color::srgb(0.2, 0.7, 1.0));

    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
        ))
        .with_children(|player| {
            player
                .spawn((
                    Head::default(),
                    Transform::from_xyz(0.0, 0.7, 0.0),
                    Visibility::default(),
                ))
                .with_children(|head| {
                    head.spawn((
                        PlayerCamera,
                        Camera3d::default(),
                        Transform::default(),
                    ));
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
