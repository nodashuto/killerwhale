use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_crosshair(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let window = window_query.single().unwrap();
    let crosshair_size = 2.0;

    commands.spawn((
        Node {
            width: Val::Px(crosshair_size),
            height: Val::Px(crosshair_size),
            position_type: PositionType::Absolute,
            left: Val::Px(window.width() / 2.0 - crosshair_size / 2.0),
            top: Val::Px(window.height() / 2.0 - crosshair_size / 2.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
    ));
}
