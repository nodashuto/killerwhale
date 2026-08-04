use bevy::{
    prelude::*,
};

use bevy_rapier3d::prelude::*;

use super::player::player;
use super::level::level;
use super::tracer::tracer;
use super::target::target;

use crate::game::hud::hud;

pub enum GameState{
    Setup,
    MainMenu,
    InGame,
    
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
	// Register your systems here
	app
	    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default()) //set up rapier
	    .add_plugins(RapierDebugRenderPlugin::default())
	    .add_plugins(level::LevelPlugin)
	    .add_plugins(hud::HUDPlugin)
	    .add_plugins(player::PlayerPlugin)
            .insert_resource(player::MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 6.0,          // default: 12.0
            })
            .add_plugins(tracer::TracerPlugin)
	    .add_plugins(target::TargetPlugin)
            //.add_systems(Startup, scene.spawn())
            //.add_systems(Update, close_on_esc)
	    ;
	    
    }        
}


/// set up a simple 3D scene
fn scene() -> impl SceneList {
    bsn_list! [
        (
            #CircularBase
            Mesh3d(asset_value(Circle::new(4.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
        ),
        (
            #Cube
            Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
            Transform::from_xyz(0.0, 0.5, 0.0)
        ),
        (
            PointLight {
                shadow_maps_enabled: true,
            }
            Transform::from_xyz(4.0, 8.0, 4.0)
        ),
        (
            Camera3d
            template_value(Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
        )
    ]
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
