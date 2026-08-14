use bevy_rapier3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct FirstPersonCharacterPlugin;

impl Plugin for FirstPersonCharacterPlugin {
    fn build(&self, app: &mut App) {
	
    }
}


// #[derive(Component)]
// struct Player;

// #[derive(Component)]
// struct FpsCamera;

// #[derive(Resource)]
// struct LookSettings {
//     sensitivity: f32,
// }

// #[derive(Resource, Default)]
// struct LookState {
//     yaw: f32,
//     pitch: f32,
// }

// #[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
// pub enum PlayerState {
//     Standing,
//     Walking,
//     Sprinting,
//     Jumping,
//     Falling,
// }



// #[derive(Component)]
// pub struct PlayerController {
//     pub velocity: Vec3,
//     pub isgrounded: bool,
//     // Time remaining in the jump/landing penalty.
//     jump_penalty_time: f32,
// }


// fn player_look (
//     mut mouse_motion: EventReader<MouseMotion>,
//     mut look: ResMut<LookState>,
//     settings: Res<LookSettings>,
//     mut player_query: Query<&mut Transform, With<Player>>,
//     mut camera_query: Query<&mut Transform, (With<FpsCamera>, Without<Player>)>,
// ) {
//     let mut mouse_delta = Vec2::ZERO;

//     for event in mouse_motion.read() {
//         mouse_delta += event.delta;
//     }

//     if mouse_delta == Vec2::ZERO {
//         return;
//     }

//     // Mouse X -> yaw
//     look.yaw -= mouse_delta.x * settings.sensitivity;

//     // Mouse Y -> pitch
//     look.pitch -= mouse_delta.y * settings.sensitivity;

//     // Prevent the camera from flipping upside down.
//     let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
//     look.pitch = look.pitch.clamp(-max_pitch, max_pitch);

//     // Rotate the player horizontally.
//     if let Ok(mut transform) = player_query.single_mut() {
//         transform.rotation = Quat::from_rotation_y(look.yaw);
//     }

//     // Rotate the camera vertically.
//     if let Ok(mut transform) = camera_query.single_mut() {
//         transform.rotation = Quat::from_rotation_x(look.pitch);
//     }
// }



// fn player_move () {


    
// }
