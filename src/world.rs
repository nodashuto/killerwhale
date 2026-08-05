use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::{
    camera::visibility::RenderLayers, color::palettes::tailwind,
    light::NotShadowCaster, 
};


use std::f32::consts::TAU;



pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
	app.add_plugins(RapierDebugRenderPlugin::default()); //activate gismo
	app.add_systems(Startup, spawn_world_model);
    }
}



fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(64.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    let grass = materials.add(Color::srgb(0.4, 1.0, 0.2));

    let house = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    let wall = meshes.add(Cuboid::new(2.0, 2.0, 0.1));

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.

    /*
     * floor(ground) 
    */
    commands.spawn((
	Mesh3d(ground),
	MeshMaterial3d(grass.clone()),
	RigidBody::Fixed,
        Collider::cuboid(64.0, 0.1, 64.0),
	));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    // commands.spawn((
    //     Mesh3d(cube.clone()),
    //     MeshMaterial3d(material.clone()),
    //     Transform::from_xyz(0.75, 1.75, 0.0),
    // ));




    commands.spawn((
	Mesh3d(house.clone()),
	MeshMaterial3d(material.clone()),
	RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
	Transform::from_xyz(4.0, -0.4, 4.0),
    ));

    commands.spawn((
	Mesh3d(house.clone()),
	MeshMaterial3d(material.clone()),
	RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
	Transform::from_xyz(4.0, -0.2, 3.0),
    ));

    // wall x z ++
        commands.spawn((
	Mesh3d(wall.clone()),
	MeshMaterial3d(material.clone()),
	RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
	Transform::from_xyz(4.0, 2.0, 3.0),
	));
}
