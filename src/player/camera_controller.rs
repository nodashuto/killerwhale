// use bevy::{input::mouse::MouseMotion, prelude::*};

// #[derive(Component)]
// pub struct CameraController {
//     /// Vertical rotation (pitch) and horizontal rotation (yaw), in degrees.
//     pub rotation: Vec2,

//     /// Maximum amount the camera can look up/down.
//     pub rotation_lock: f32,

//     /// How much mouse movement affects camera rotation.
//     pub sensitivity: f32,
// }

// pub fn update_camera_controller(
//     mut accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
//     mut camera_query: Query<(&mut CameraController, &mut Transform)>,
// ) {
//     let Ok((mut controller, mut transform)) = camera_query.get_single_mut() else {
//         return;
//     };

//     // Update the camera's rotation based on mouse movement.
//     for motion in accumulated_mouse_motion.read() {
//         let mouse_delta = motion.delta * controller.sensitivity;

//         // Horizontal mouse movement controls yaw.
//         controller.rotation.y -= mouse_delta.x;

//         // Vertical mouse movement controls pitch.
//         controller.rotation.x -= mouse_delta.y;

//         // Prevent the camera from flipping upside down.
//         controller.rotation.x = controller.rotation.x.clamp(
//             -controller.rotation_lock,
//             controller.rotation_lock,
//         );
//     }

//     // Convert the pitch and yaw angles into quaternions.
//     let yaw = Quat::from_rotation_y(controller.rotation.y.to_radians());
//     let pitch = Quat::from_rotation_x(controller.rotation.x.to_radians());

//     // Apply the combined rotation to the camera.
//     transform.rotation = yaw * pitch;
// }
