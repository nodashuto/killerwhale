use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

//use super::{camera_controller, input::*, player_movement::*, player_shooting::{update_player, TracerSpawnSpot}};
//use crate::game::{math::coordinates::blender_to_world, shooting};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
}


#[derive(Component)]
pub struct Player {
    pub velocity : Vec3,
    pub gravity : f32,
    pub speed : f32,
}

fn init_player(mut commands: Commands,asset_server : Res<AssetServer>) {
}
