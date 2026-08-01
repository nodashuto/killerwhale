const GRAVITY: f32 = -30.0;
const JUMP_SPEED: f32 = 10.0;
const GROUND_Y: f32 = 0.0;

use std::f32::consts::FRAC_PI_2;

use crate::game::target::target::{Target, Health};



use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::math::*;

use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    color::palettes::*,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*,
};

use crate::game::tracer;

pub mod prelude {
    pub use crate::*;
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

/// Key configuration

#[derive(Resource)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
    pub sprint: KeyCode,
    pub crouch: KeyCode,
}

/// https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html
impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ShiftLeft,
            toggle_grab_cursor: KeyCode::Escape,
	    sprint: KeyCode::ShiftLeft,
	    crouch: KeyCode::ControlLeft,
        }
    }
}

/// Used in queries when you want flycams and not other cameras
/// A marker component used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct FlyCam;


#[derive(Component)]
pub struct Player;


#[derive(Component)]
struct PlayerCamera;

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

/// start: https://bevy.org/examples/camera/first-person-view-model/

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

/// ends

/// Spawns the `Camera3dBundle` to be controlled
fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 1.0));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));
    commands.spawn((
        //Camera3d::default(),
        Player,
	PlayerPhysics::default(),
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
	children![
            (
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 80.0_f32.to_radians(),
                    ..default()
                }),
            ),
            // Spawn view model camera.
            (
		PlayerCamera,
		Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ),
            // Spawn the player's right arm.
            (
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                
		Transform {
		    translation: Vec3::new(-0.1, -0.4, -0.25),
		    rotation: Quat::from_rotation_y(std::f32::consts::PI / -6.0),
		    scale: Vec3::new(1.0, 1.0, 1.0),
		},
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ),
	    (
		Mesh3d(asset_server.load("models/sniper-0001.glb#Mesh0/Primitive0")),
		MeshMaterial3d(materials.add(StandardMaterial {base_color: Color::BLACK,..default()})),
		Transform {
		    translation: Vec3::new(0.2, -0.3, -0.7),
		    rotation: Quat::from_rotation_y(std::f32::consts::PI),
		    scale: Vec3::new(1.0, 1.0, 6.0),
		},
		// Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,		     
		
	    ),
        ],
    ));
}

/// Handles keyboard input and movement
fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(&Player, &mut Transform, &mut PlayerPhysics)>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    for (_camera, mut transform, mut physics) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            match primary_cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let key = *key;
                    if key == key_bindings.move_forward {
                        velocity += forward;
                    } else if key == key_bindings.move_backward {
                        velocity -= forward;
                    } else if key == key_bindings.move_left {
                        velocity -= right;
                    } else if key == key_bindings.move_right {
                        velocity += right;
                    } else if key == key_bindings.move_ascend {
                        //velocity += Vec3::Y;
                    } else if key == key_bindings.move_descend {
                        //velocity -= Vec3::Y;
                    }
		    // Jump
		    if keys.just_pressed(key_bindings.move_ascend) && physics.is_grounded {
			physics.vertical_velocity = JUMP_SPEED;
			physics.is_grounded = false;
		    }
                }
            }
        }

        velocity = velocity.normalize_or_zero();

        //transform.translation += velocity * time.delta_secs() * settings.speed;

	// Sprint
	
	if keys.pressed(key_bindings.move_forward) & keys.pressed(key_bindings.sprint)  {
	    transform.translation += velocity * settings.speed * time.delta_secs() * 1.5; // sprint is 1.5x faster
	}


	 // Horizontal movement
	 transform.translation += velocity * settings.speed * time.delta_secs();

	// Gravity
	physics.vertical_velocity += GRAVITY * time.delta_secs();

	// Vertical movement
	transform.translation.y += physics.vertical_velocity * time.delta_secs();

	// Ground collision
	if transform.translation.y <= GROUND_Y {
            transform.translation.y = GROUND_Y;
            physics.vertical_velocity = 0.0;
            physics.is_grounded = true;
	}
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut state: MessageReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(window) = primary_window.single() {
        for mut transform in query.iter_mut() {
            for ev in state.read() {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match primary_cursor_options.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if keys.just_pressed(key_bindings.toggle_grab_cursor) {
        toggle_grab_cursor(primary_cursor_options);
    }
}

// Grab cursor when an entity with FlyCam is added
fn initial_grab_on_flycam_spawn(
    query_added: Query<Entity, Added<Player>>,
    primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if query_added.is_empty() {
        return;
    }

    toggle_grab_cursor(primary_cursor_options);
}

/// Contains everything needed to add first-person fly camera behavior to your game
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        common_build(app);
	//app.add_plugins(tracer::tracer::TracerPlugin); //added
	app.add_systems(Startup, (
	    //spawn_view_model,
	    setup_player,
	));
	app.add_systems(Update, update_player); // added myself
	app.add_systems(Update, fire_weapon);
	
    }
}

/// Same as [`PlayerPlugin`] but does not spawn a camera
pub struct NoCameraPlayerPlugin;
impl Plugin for NoCameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        common_build(app);
    }
}

/// Common build steps for both PlayerPlugin and NoCameraPlayerPlugin
fn common_build(app: &mut App) {
    app.init_resource::<MovementSettings>()
        .init_resource::<KeyBindings>()
        .add_systems(Startup, initial_grab_cursor)
        .add_systems(Startup, initial_grab_on_flycam_spawn)
        .add_systems(Update, player_move)
        .add_systems(Update, player_look)
        .add_systems(Update, cursor_grab);
}


/// Add a player physics component
#[derive(Component)]
pub struct PlayerPhysics {
    pub vertical_velocity: f32,
    pub is_grounded: bool,
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        Self {
            vertical_velocity: 0.0,
            is_grounded: false,
	    
        }
    }
}



fn update_player(
    buttons: Res<ButtonInput<MouseButton>>,
) {
    for button in buttons.get_pressed() {
        //println!("{:?} is currently held down", button);
    }
    for button in buttons.get_just_pressed() {
        //println!("{:?} was pressed", button);
    }
    for button in buttons.get_just_released() {
        //println!("{:?} was released", button);
    }
}


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
        children![
            (
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 64.0_f32.to_radians(),
                    ..default()
                }),
            ),
             //Spawn view model camera.
            (
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
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


fn fire_weapon(
    buttons: Res<ButtonInput<MouseButton>>,
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    rapier_context: ReadRapierContext,
    mut health_query: Query<&mut Health>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let transform = camera.single().unwrap();

    let origin = transform.translation();
    let direction = transform.forward();

    let max_distance = 1000.0;

    if let Ok(ctx) = rapier_context.single() {
        if let Some((entity, toi)) = ctx.cast_ray_and_get_normal(
            origin,
            *direction,
            max_distance,
            true,
            QueryFilter::default(),
        ) {
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
