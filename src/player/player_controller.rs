use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player::PlayerMovementMode;

/// Maximum normal movement speed.
pub const PLAYER_SPEED: f32 = 14.826;

/// Sprint movement speed.
pub const PLAYER_SPRINTING_SPEED: f32 = 17.239;

/// Gravity in your current world scale.
pub const PLAYER_GRAVITY: f32 = 40.32;

/// Jump height in world units.
pub const JUMP_HEIGHT: f32 = 1.8;

/// Maximum falling velocity.
pub const MAX_FALL_SPEED: f32 = 60.0;

/// Time after leaving a ledge during which jumping is still allowed.
///
/// This is the JS controller's `coyoteTime`.
pub const COYOTE_TIME: f32 = 0.10;

/// How long a jump input is remembered.
///
/// This is the JS controller's `jumpBufferTime`.
pub const JUMP_BUFFER_TIME: f32 = 0.12;

// -----------------------------------------------------------------------------
// Sprint
// -----------------------------------------------------------------------------

pub const MAX_SPRINT_TIME: f32 = 2.0;
pub const SPRINT_RECHARGE_PAUSE: f32 = 0.3;

// -----------------------------------------------------------------------------
// Movement
// -----------------------------------------------------------------------------

/// Ground acceleration.
///
/// Higher values make the player reach maximum speed faster.
pub const GROUND_ACCEL: f32 = 50.0;

/// Air acceleration.
///
/// Deliberately lower than ground acceleration.
pub const AIR_ACCEL: f32 = 2.0;

/// Ground friction.
pub const GROUND_FRICTION: f32 = 24.0;

/// Minimum speed used by the friction calculation.
pub const STOP_SPEED: f32 = 16.0;

/// ADS movement multiplier.
pub const ADS_SPEED_MULTIPLIER: f32 = 0.6;

// -----------------------------------------------------------------------------
// Jump penalty
// -----------------------------------------------------------------------------

pub const JUMP_PENALTY_DURATION: f32 = 0.8;
pub const JUMP_SLOWDOWN_SPEED: f32 = 0.5;
pub const JUMP_LAND_SLOWDOWN_TIME: f32 = 1.7;
pub const JUMP_REJUMP_FACTOR: f32 = 2.5;

// -----------------------------------------------------------------------------
// Player controller
// -----------------------------------------------------------------------------

#[derive(Debug, Component)]
pub struct PlayerPhysicsController {
    /// Current player velocity.
    pub velocity: Vec3,

    /// Whether the player was detected on the ground.
    pub isgrounded: bool,

    /// Jump/landing movement penalty timer.
    pub jump_penalty_time: f32,

    // -------------------------------------------------------------------------
    // Sprint
    // -------------------------------------------------------------------------
    pub sprint_remaining: f32,
    pub sprint_recharge_delay: f32,
    pub is_sprinting: bool,

    // -------------------------------------------------------------------------
    // ADS
    // -------------------------------------------------------------------------
    pub is_ads: bool,

    // -------------------------------------------------------------------------
    // JS controller features
    // -------------------------------------------------------------------------
    /// Time remaining in coyote time.
    pub coyote_timer: f32,

    /// Time remaining in jump input buffer.
    pub jump_buffer_timer: f32,

    /// Whether the previous frame had the player grounded.
    pub was_grounded: bool,
}

impl Default for PlayerPhysicsController {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,

            isgrounded: false,

            jump_penalty_time: 0.0,

            sprint_remaining: MAX_SPRINT_TIME,
            sprint_recharge_delay: 0.0,
            is_sprinting: false,

            is_ads: false,

            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
            was_grounded: false,
        }
    }
}

// -----------------------------------------------------------------------------
// Plugin
// -----------------------------------------------------------------------------

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_grounded, player_controller).chain());
    }
}

// -----------------------------------------------------------------------------
// Ground state
// -----------------------------------------------------------------------------

fn update_grounded(
    mut query: Query<(
        &mut PlayerPhysicsController,
        &KinematicCharacterControllerOutput,
    )>,
) {
    for (mut player, output) in &mut query {
        player.was_grounded = player.isgrounded;
        player.isgrounded = output.grounded;
    }
}

// -----------------------------------------------------------------------------
// Main controller
// -----------------------------------------------------------------------------

fn player_controller(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,

    mut query: Query<(
        &Transform,
        &mut KinematicCharacterController,
        &mut PlayerPhysicsController,
        &PlayerMovementMode,
    )>,
) {
    let dt = time.delta_secs();

    for (transform, mut character_controller, mut player, movement_mode) in &mut query {
        // -------------------------------------------------------------
        // NoClip
        // -------------------------------------------------------------

        if *movement_mode == PlayerMovementMode::NoClip {
            character_controller.translation = None;
            player.velocity = Vec3::ZERO;
            continue;
        }

        // -------------------------------------------------------------
        // Input
        // -------------------------------------------------------------

        let input = movement_input(&keyboard);

        // Player yaw controls movement direction.
        let mut wish_dir = transform.rotation * input;

        // Movement must stay horizontal.
        wish_dir.y = 0.0;

        if wish_dir.length_squared() > 0.000001 {
            wish_dir = wish_dir.normalize();
        }

        // -------------------------------------------------------------
        // ADS
        // -------------------------------------------------------------

        player.is_ads = mouse.pressed(MouseButton::Right);

        let is_ads = player.is_ads;

        // -------------------------------------------------------------
        // Sprint
        // -------------------------------------------------------------

        let wants_sprint = wants_to_sprint(&keyboard, input);

        update_sprint(&mut player, wants_sprint, is_ads, dt);

        // -------------------------------------------------------------
        // Jump input buffer
        // -------------------------------------------------------------

        if keyboard.just_pressed(KeyCode::Space) {
            player.jump_buffer_timer = JUMP_BUFFER_TIME;
        } else {
            player.jump_buffer_timer = (player.jump_buffer_timer - dt).max(0.0);
        }

        // -------------------------------------------------------------
        // Coyote time
        // -------------------------------------------------------------

        if player.isgrounded {
            player.coyote_timer = COYOTE_TIME;
        } else {
            player.coyote_timer = (player.coyote_timer - dt).max(0.0);
        }

        // -------------------------------------------------------------
        // Jump
        // -------------------------------------------------------------

        try_jump(&mut player);

        // -------------------------------------------------------------
        // Jump penalty
        // -------------------------------------------------------------

        player.jump_penalty_time = (player.jump_penalty_time - dt).max(0.0);

        let penalty_scale = if player.jump_penalty_time > 0.0 {
            JUMP_SLOWDOWN_SPEED
        } else {
            1.0
        };

        // -------------------------------------------------------------
        // Movement speed
        // -------------------------------------------------------------

        let base_speed = if player.is_sprinting {
            PLAYER_SPRINTING_SPEED
        } else {
            PLAYER_SPEED
        };

        let ads_multiplier = if player.is_ads {
            ADS_SPEED_MULTIPLIER
        } else {
            1.0
        };

        // Forward = 100%
        // Sideways = 80%
        // Backward = 70%
        let movement_multiplier = if keyboard.pressed(KeyCode::KeyS) {
            0.7
        } else if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::KeyD) {
            0.8
        } else {
            1.0
        };

        let wish_speed = if wish_dir != Vec3::ZERO {
            base_speed * penalty_scale * ads_multiplier * movement_multiplier
        } else {
            0.0
        };

        // -------------------------------------------------------------
        // Ground movement
        // -------------------------------------------------------------

        if player.isgrounded {
            ground_move(&mut player.velocity, wish_dir, wish_speed, dt);
        } else {
            // ---------------------------------------------------------
            // Air movement
            // ---------------------------------------------------------

            air_move(&mut player.velocity, wish_dir, wish_speed, dt);

            // ---------------------------------------------------------
            // Gravity
            // ---------------------------------------------------------

            player.velocity.y -= PLAYER_GRAVITY * dt;

            // Limit fall speed.
            player.velocity.y = player.velocity.y.max(-MAX_FALL_SPEED);
        }

        // -------------------------------------------------------------
        // Send movement to Rapier
        // -------------------------------------------------------------

        character_controller.translation = Some(player.velocity * dt);
    }
}

// -----------------------------------------------------------------------------
// WASD input
// -----------------------------------------------------------------------------

fn movement_input(keyboard: &ButtonInput<KeyCode>) -> Vec3 {
    let mut input = Vec3::ZERO;

    // W
    if keyboard.pressed(KeyCode::KeyW) {
        input.z -= 1.0;
    }

    // S
    if keyboard.pressed(KeyCode::KeyS) {
        input.z += 1.0;
    }

    // A
    if keyboard.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }

    // D
    if keyboard.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    // Same behavior as the JS controller:
    // diagonal movement is normalized.
    if input.length_squared() > 1.0 {
        input = input.normalize();
    }

    input
}

// -----------------------------------------------------------------------------
// Ground movement
// -----------------------------------------------------------------------------

fn ground_move(velocity: &mut Vec3, wish_dir: Vec3, wish_speed: f32, dt: f32) {
    // JS controller:
    //
    // ground movement uses acceleration and friction.

    friction(velocity, GROUND_FRICTION, STOP_SPEED, dt);

    accelerate(velocity, wish_dir, wish_speed, GROUND_ACCEL, dt);

    // Never keep downward velocity while grounded.
    if velocity.y < 0.0 {
        velocity.y = 0.0;
    }
}

// -----------------------------------------------------------------------------
// Air movement
// -----------------------------------------------------------------------------

fn air_move(velocity: &mut Vec3, wish_dir: Vec3, wish_speed: f32, dt: f32) {
    accelerate(velocity, wish_dir, wish_speed, AIR_ACCEL, dt);
}

// -----------------------------------------------------------------------------
// Acceleration
// -----------------------------------------------------------------------------

fn accelerate(velocity: &mut Vec3, wish_dir: Vec3, wish_speed: f32, acceleration: f32, dt: f32) {
    if wish_dir == Vec3::ZERO || wish_speed <= 0.0 {
        return;
    }

    // Current velocity in the direction the player wants to move.
    let current_speed = velocity.dot(wish_dir);

    // How much speed we still need.
    let add_speed = wish_speed - current_speed;

    if add_speed <= 0.0 {
        return;
    }

    // Same basic acceleration model as the JS controller.
    let accel_speed = acceleration * dt * wish_speed;

    let accel_speed = accel_speed.min(add_speed);

    *velocity += wish_dir * accel_speed;
}

// -----------------------------------------------------------------------------
// Friction
// -----------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------
// Jump
// -----------------------------------------------------------------------------

fn try_jump(player: &mut PlayerPhysicsController) {
    if player.jump_buffer_timer <= 0.0 {
        return;
    }

    // The JS controller allows jumping either while grounded
    // or shortly after leaving the ground.
    let can_jump = player.isgrounded || player.coyote_timer > 0.0;

    if !can_jump {
        return;
    }

    // v = sqrt(2gh)
    let jump_speed = (2.0 * PLAYER_GRAVITY * JUMP_HEIGHT).sqrt();

    let land_factor = get_jump_land_factor(player.jump_penalty_time);

    player.velocity.y = jump_speed / land_factor.sqrt();

    player.isgrounded = false;

    // Consume jump input.
    player.jump_buffer_timer = 0.0;

    // Start jump penalty.
    player.jump_penalty_time = JUMP_PENALTY_DURATION;

    // Consume coyote time.
    player.coyote_timer = 0.0;
}

// -----------------------------------------------------------------------------
// Jump penalty
// -----------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------
// Sprint
// -----------------------------------------------------------------------------

fn update_sprint(player: &mut PlayerPhysicsController, wants_sprint: bool, is_ads: bool, dt: f32) {
    // ADS always cancels sprint.
    if is_ads {
        if player.is_sprinting {
            player.is_sprinting = false;
            player.sprint_recharge_delay = SPRINT_RECHARGE_PAUSE;
        }

        return;
    }

    // -------------------------------------------------------------
    // Currently sprinting
    // -------------------------------------------------------------

    if player.is_sprinting {
        if wants_sprint && player.sprint_remaining > 0.0 {
            player.sprint_remaining = (player.sprint_remaining - dt).max(0.0);

            if player.sprint_remaining <= 0.0 {
                player.is_sprinting = false;
                player.sprint_recharge_delay = SPRINT_RECHARGE_PAUSE;
            }
        } else {
            player.is_sprinting = false;
            player.sprint_recharge_delay = SPRINT_RECHARGE_PAUSE;
        }

        return;
    }

    // -------------------------------------------------------------
    // Not sprinting
    // -------------------------------------------------------------

    if !wants_sprint {
        if player.sprint_recharge_delay > 0.0 {
            player.sprint_recharge_delay = (player.sprint_recharge_delay - dt).max(0.0);
        } else {
            player.sprint_remaining = (player.sprint_remaining + dt).min(MAX_SPRINT_TIME);
        }
    }

    // -------------------------------------------------------------
    // Start sprint
    // -------------------------------------------------------------

    if wants_sprint && player.sprint_recharge_delay <= 0.0 && player.sprint_remaining > 0.0 {
        player.is_sprinting = true;
    }
}

// -----------------------------------------------------------------------------
// Sprint input
// -----------------------------------------------------------------------------

fn shift_held(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight)
}

fn wants_to_sprint(keyboard: &ButtonInput<KeyCode>, input: Vec3) -> bool {
    // Match your existing behavior:
    // Shift + W.
    shift_held(keyboard) && input.z < 0.0
}
