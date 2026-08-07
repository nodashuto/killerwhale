use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy::color::palettes::*;

use crate::Health;



pub struct ShootingTargetPlugin;



impl Plugin for ShootingTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_dead_shooting_targets);
    }
}

#[derive(Component)]
pub struct ShootingTarget;



pub fn spawn_shooting_target(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    let radius = 0.05;
    let mesh = meshes.add(Sphere::new(radius).mesh().ico(7).unwrap());

    let material = materials.add(StandardMaterial {
	base_color: Color::srgb(118.0 / 255.0, 205.0 / 255.0, 38.0 / 255.0), //Color::srgb(1.0, 0.2, 0.2),
        ..default()
    });

    commands.spawn((
        ShootingTarget,
        Name::new("Shooting_Target"),
        Health {
            current: 50.0,
            max: 50.0,
        },
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Fixed,
        Collider::ball(radius + 0.01),
    ));
}

fn despawn_dead_shooting_targets(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<ShootingTarget>>,
) {
    for (entity, health) in &query {
        if health.current <= 0.0 {
            println!("Shooting Target destroyed");
            commands.entity(entity).despawn();
        }
    }
}
