use bevy::prelude::*;

use bevy::window::PrimaryWindow;

use super::crosshair::{spawn_crosshair, toggle_and_animate_crosshair};

use crate::player::player::EquippedWeapon;
use crate::weapon::weapon::Weapon;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_hud, spawn_crosshair))
            .add_systems(Update, (update_weapon_hud, toggle_and_animate_crosshair));
    }
}

#[derive(Component)]
struct WeaponHudText; // marker component for ammo ui

#[derive(Component)]
struct NoAmmoText;

#[derive(Component)]
struct ReloadHintText;

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Text::new(" 0 / 0"),
        WeaponHudText,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(30.0),
            bottom: Val::Px(30.0),

            padding: UiRect::all(Val::Px(10.0)),

            ..default()
        },
        TextFont {
            font_size: FontSize::Px(45.0),
            ..default()
        },
        //BackgroundColor(Color::BLACK),
	//BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.4)),
    ));

    commands.spawn((
        Text::new(""),
        NoAmmoText,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(25.0),
            left: Val::Percent(46.0),
            ..default()
        },
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
	TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Visibility::Hidden,
    ));

    

    // commands
    //     .spawn((
    //         ReloadHintText,
    //         Node {
    //             position_type: PositionType::Absolute,
    //             bottom: Val::Percent(25.0),
    //             left: Val::Percent(45.0),
    //             ..default()
    //         },
    //         Visibility::Hidden,
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn((
    //             TextSpan::new("Press "),
    //             TextColor(Color::WHITE),
    //             TextFont {
    //                 font_size: FontSize::Px(24.0),
    //                 ..default()
    //             },
    //         ));

    //         parent.spawn((
    //             TextSpan::new("R"),
    //             TextColor(Color::srgb(1.0, 1.0, 0.0)),
    //             TextFont {
    //                 font_size: FontSize::Px(24.0),
    //                 ..default()
    //             },
    //         ));

    //         parent.spawn((
    //             TextSpan::new(" to reload"),
    //             TextColor(Color::WHITE),
    //             TextFont {
    //                 font_size: FontSize::Px(24.0),
    //                 ..default()
    //             },
    //         ));
    //     });

    commands
    .spawn((
        Text::new(""),
        ReloadHintText,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Percent(40.0),
            bottom: Val::Percent(35.0),
            ..default()
        },
        Visibility::Hidden,
    ))
    .with_children(|parent| {
        parent.spawn((
            TextSpan::new("Press "),
            TextColor(Color::WHITE),
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
        ));

        parent.spawn((
            TextSpan::new("R"),
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
        ));

        parent.spawn((
            TextSpan::new(" to reload"),
            TextColor(Color::WHITE),
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
        ));
    });
    
}

fn update_weapon_hud(
    weapon_query: Query<&Weapon, With<EquippedWeapon>>,

    mut queries: ParamSet<(
        Query<&mut Text, With<WeaponHudText>>,
        Query<(&mut Text, &mut Visibility), With<NoAmmoText>>,
        Query<&mut Visibility, With<ReloadHintText>>,
    )>,
) {
    let Ok(weapon) = weapon_query.single() else {
        return;
    };

    // -----------------------------------------
    // WEAPON / AMMO HUD
    // -----------------------------------------

    if let Ok(mut text) = queries.p0().single_mut() {
        text.0 = format!(
            " {} / {}",
             weapon.ammo_in_magazine, weapon.reserve_ammo,
        );
    }

    // -----------------------------------------
    // AMMO WARNING
    // -----------------------------------------

    let mut binding = queries.p1();
    let Ok((mut message, mut visibility)) = binding.single_mut() else {
        return;
    };

    let total_ammo = weapon.ammo_in_magazine + weapon.reserve_ammo;

    if total_ammo == 0 {
        message.0 = "No Ammo".to_string();
        *visibility = Visibility::Visible;
    } else if total_ammo < 10 {
        message.0 = "Low Ammo".to_string();
        *visibility = Visibility::Visible;
    } else {
        message.0.clear();
        *visibility = Visibility::Hidden;
    }

    // -----------------------------------------
    // RELOAD HINT
    // -----------------------------------------

    let mut p2 = queries.p2();

    if let Ok(mut visibility) = p2.single_mut() {
        if weapon.ammo_in_magazine < 3 && weapon.reserve_ammo > 0 {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
