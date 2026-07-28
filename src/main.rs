use bevy::prelude::*;


// Place this before main function in main.rs
#[derive(Component)]
struct Player;


fn main() {
    App::new()
	.add_plugins(DefaultPlugins)
	.add_systems(Startup, setup)
	.add_systems(Update, close_on_esc)
	.add_systems(Update, move_player)
	.run();

}

// Replace existing setup function in main.rs with the following code
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);	
    
    // Code Update Alert
    // Append the following lines to your setup function.
    commands.spawn((
        Text2d::new("X"),
        TextFont {
            font_size: FontSize::Px(20.0),	
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::ZERO),
        Player,
    ));
}

// Allow player to move
fn move_player(
    // "Bevy, give me keyboard input"
    input: Res<ButtonInput<KeyCode>>,           
    // "Bevy, give me the game timer"
    time: Res<Time>,                            
    // "Bevy, give me the player's position"
    mut player_transform: Single<&mut Transform, With<Player>>, 
) {
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let speed = 300.0; // pixels per second
        let delta = direction.normalize() * speed * time.delta_secs();
        player_transform.translation.x += delta.x;
        player_transform.translation.y += delta.y;
    }
}


// Press Esc to quit game
pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
