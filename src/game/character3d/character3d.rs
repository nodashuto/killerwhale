use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::math::*;


/// For more infomation, see: https://rapier.rs/docs/user_guides/bevy_plugin/character_controller/
fn setup_physics(mut commands: Commands) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(Transform::default())
        .insert(KinematicCharacterController {
            ..KinematicCharacterController::default()
        });
}

fn update_system(time: Res<Time>, mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec3::new(1.0, -5.0, -1.0) * time.delta_secs());
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

fn character_movement() {
    
}

fn jump() {
    
}

