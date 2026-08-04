use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion, light::NotShadowCaster, prelude::*,
};
use bevy::color::*;

// This line tells the compiler to include the code it finds in src/game.rs
//pub mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
    //.add_plugins(game::game::GamePlugin)
    //.add_systems(Startup, (init_level, spawn_lights))
        .add_systems(Startup, init_level)
	.add_systems(Startup, setup)
	.add_systems(Startup, setup_physics)
	.add_systems(Update, update_system)
	.add_systems(Update, read_result_system)
        .run();
}

#[derive(Debug, Component)]
struct Player;



fn init_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
     /*
     * Ground
     */
    let ground_size = 100.1;
    let ground_height = 0.5;

    commands.spawn((
        Transform::from_xyz(0.0, -ground_height, 0.0),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    // spawn floor
    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone()), Transform::from_xyz(0.0, 0.1, 0.0),));

    
}


fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::NEUTRAL_300),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        //RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

/// For more infomation, see: https://rapier.rs/docs/user_guides/bevy_plugin/character_controller/
fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    /*
     * Spawn Player with CharacterController
     */

    commands.spawn((
        //RigidBody::KinematicPositionBased,
	//Player,
	RigidBody::KinematicPositionBased,
        Transform::from_xyz(0.0, 4.1, 0.0),
	Visibility::default(),
	Collider::cuboid(1.0, 2.0, 1.0),
	ColliderDebugColor(Srgba::rgb(0.5, 0.5, 0.5).into()),
	KinematicCharacterController{
            ..KinematicCharacterController::default()
        },
	Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
    ));

}

/* , With<Player>
*/
fn update_system(time: Res<Time>, mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec3::new(0.0, -1.0, 0.0) * time.delta_secs());
    }
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            entity, output.effective_translation, output.grounded
        );
    }
}
#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn character_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
     mut controllers: Query<&mut KinematicCharacterController>,
) {
    for mut controller in controllers.iter_mut() {
	let mut movement = Vec3::ZERO;
        let forward = Vec3::new(1.0, 0., 0.0);
        let right = Vec3::new(0.0, 0., 1.0);


        

	if keys.pressed(KeyCode::KeyW){
            movement = forward;
	}
	if keys.pressed(KeyCode::KeyA){

            movement = right;
	}
	if keys.pressed(KeyCode::KeyS){
            movement = -forward;
	    
	}
	if keys.pressed(KeyCode::KeyD){
            movement = -right;
	}

	movement = movement.normalize_or_zero();
	controller.translation = Some(
	    movement * time.delta_secs()
	);

	
	controller.translation = Some(Vec3::new(0.0, -0.1, 0.0) * time.delta_secs());
    }
}



fn jump() {
    
}




