use bevy::prelude::*;
use bevy::window::PrimaryWindow;


const ARM_LENGTH: f32 = 8.0;
const THICKNESS: f32 = 2.0;
const OPEN_GAP: f32 = 40.0;
const CLOSED_GAP: f32 = 2.0;
const CROSSHAIR_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub enum CrosshairArm {
    Top,
    Bottom,
    Left,
    Right,
}


pub fn spawn_crosshair(window_query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    let window = window_query.single().unwrap();

    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;

    commands
        .spawn((
            Crosshair,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(center_x),
                top: Val::Px(center_y),
                ..default()
            },
        ))
        .with_children(|parent| {
            let color = BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.6));

            parent.spawn((
                CrosshairArm::Top,
                Node {
                    width: Val::Px(THICKNESS),
                    height: Val::Px(ARM_LENGTH),
                    position_type: PositionType::Absolute,
                    left: Val::Px(-THICKNESS / 2.0),
                    top: Val::Px(-(OPEN_GAP / 2.0 + ARM_LENGTH)),
                    ..default()
                },
                color.clone(),
            ));

            parent.spawn((
                CrosshairArm::Bottom,
                Node {
                    width: Val::Px(THICKNESS),
                    height: Val::Px(ARM_LENGTH),
                    position_type: PositionType::Absolute,
                    left: Val::Px(-THICKNESS / 2.0),
                    top: Val::Px(OPEN_GAP / 2.0),
                    ..default()
                },
                color.clone(),
            ));

            parent.spawn((
                CrosshairArm::Left,
                Node {
                    width: Val::Px(ARM_LENGTH),
                    height: Val::Px(THICKNESS),
                    position_type: PositionType::Absolute,
                    left: Val::Px(-(OPEN_GAP / 2.0 + ARM_LENGTH)),
                    top: Val::Px(-THICKNESS / 2.0),
                    ..default()
                },
                color.clone(),
            ));

            parent.spawn((
                CrosshairArm::Right,
                Node {
                    width: Val::Px(ARM_LENGTH),
                    height: Val::Px(THICKNESS),
                    position_type: PositionType::Absolute,
                    left: Val::Px(OPEN_GAP / 2.0),
                    top: Val::Px(-THICKNESS / 2.0),
                    ..default()
                },
                color,
            ));
        });
}

pub fn toggle_and_animate_crosshair(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut crosshair_query: Query<&mut Visibility, With<Crosshair>>,
    mut arm_query: Query<(&CrosshairArm, &mut Node)>,
) {
    let _left_pressed = mouse.pressed(MouseButton::Left);
    let right_pressed = mouse.pressed(MouseButton::Right);

    let target_gap = if right_pressed {
        CLOSED_GAP
    } else {
        OPEN_GAP
    };

    let amount = CROSSHAIR_SPEED * time.delta_secs();

    let mut reached_target = true;

    for (arm, mut node) in &mut arm_query {
        match arm {
            CrosshairArm::Top => {
                let target = -(target_gap / 2.0 + ARM_LENGTH);

                if let Val::Px(current) = node.top {
                    let new_value = move_towards(current, target, amount);
                    node.top = Val::Px(new_value);

                    if (new_value - target).abs() > 0.01 {
                        reached_target = false;
                    }
                }
            }

            CrosshairArm::Bottom => {
                let target = target_gap / 2.0;

                if let Val::Px(current) = node.top {
                    let new_value = move_towards(current, target, amount);
                    node.top = Val::Px(new_value);

                    if (new_value - target).abs() > 0.01 {
                        reached_target = false;
                    }
                }
            }

            CrosshairArm::Left => {
                let target = -(target_gap / 2.0 + ARM_LENGTH);

                if let Val::Px(current) = node.left {
                    let new_value = move_towards(current, target, amount);
                    node.left = Val::Px(new_value);

                    if (new_value - target).abs() > 0.01 {
                        reached_target = false;
                    }
                }
            }

            CrosshairArm::Right => {
                let target = target_gap / 2.0;

                if let Val::Px(current) = node.left {
                    let new_value = move_towards(current, target, amount);
                    node.left = Val::Px(new_value);

                    if (new_value - target).abs() > 0.01 {
                        reached_target = false;
                    }
                }
            }
        }
    }

    for mut visibility in &mut crosshair_query {
        if right_pressed && reached_target {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

fn _animate_crosshair(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&CrosshairArm, &mut Node)>,
) {
    let target_gap = if mouse.pressed(MouseButton::Right) {
        CLOSED_GAP
    } else {
        OPEN_GAP
    };

    let amount = CROSSHAIR_SPEED * time.delta_secs();

    for (arm, mut node) in &mut query {
        match arm {
            CrosshairArm::Top => {
                let target = -(target_gap / 2.0 + ARM_LENGTH);

                if let Val::Px(current) = node.top {
                    node.top = Val::Px(move_towards(current, target, amount));
                }
            }
            CrosshairArm::Bottom => {
                let target = target_gap / 2.0;

                if let Val::Px(current) = node.top {
                    node.top = Val::Px(move_towards(current, target, amount));
                }
            }
            CrosshairArm::Left => {
                let target = -(target_gap / 2.0 + ARM_LENGTH);

                if let Val::Px(current) = node.left {
                    node.left = Val::Px(move_towards(current, target, amount));
                }
            }
            CrosshairArm::Right => {
                let target = target_gap / 2.0;

                if let Val::Px(current) = node.left {
                    node.left = Val::Px(move_towards(current, target, amount));
                }
            }
        }
    }
}

pub fn _toggle_crosshair(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Visibility, With<Crosshair>>,
) {
    for mut visibility in &mut query {
        *visibility = if mouse.pressed(MouseButton::Right) {
            Visibility::Visible
        } else {
            Visibility::Visible
        };
    }
}

fn move_towards(current: f32, target: f32, amount: f32) -> f32 {
    if (target - current).abs() <= amount {
        target
    } else {
        current + (target - current).signum() * amount
    }
}
