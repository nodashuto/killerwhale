use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy::color::palettes::*;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_dead_targets);
    }
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

pub fn spawn_target(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    let mesh = meshes.add(Sphere::new(0.1).mesh().ico(7).unwrap());

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.2, 0.2),
        ..default()
    });

    commands.spawn((
        Target,
        Name::new("Target"),
        Health {
            current: 100.0,
            max: 100.0,
        },
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
        RigidBody::Fixed,
        Collider::cuboid(0.1, 0.1, 0.1),
    ));
}

fn despawn_dead_targets(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<Target>>,
) {
    for (entity, health) in &query {
        if health.current <= 0.0 {
            println!("Target destroyed");
            commands.entity(entity).despawn();
        }
    }
}
