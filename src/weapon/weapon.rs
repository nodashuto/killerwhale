use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::player::Player;
use crate::player::player::PlayerCamera;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_plugin_loaded);
        app.add_systems(Update, fire_weapon);
    }
}

/// prints plugin is loaded in console
fn print_plugin_loaded() {
    println!("weapon plugin is loaded");
}

/// stores weapon name, damage
#[derive(Component, Debug)]
pub struct Weapon {
    // the name never changes
    pub name: &'static str,
    pub damage: f32,
    pub range: f32,
}

#[derive(Debug)]
pub struct BulletHit {
    pub entity: Entity,
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

/// Reusable hitscan/bullet hitbox system.
///
/// Performs a raycast from `origin` in `direction` and returns
/// information about the first hit.
///
/// `exclude` is typically the entity that fired the weapon.
fn bullet_fire(
    rapier_context: &RapierContext,
    origin: Vec3,
    direction: Vec3,
    max_distance: f32,
    exclude: Entity,
) -> Option<BulletHit> {
    let direction = direction.normalize();

    let filter = QueryFilter::default().exclude_rigid_body(exclude);

    if let Some((entity, toi)) =
        rapier_context.cast_ray(origin, direction, max_distance, true, filter)
    {
        let hit_position = origin + direction * toi;

        // If you want the hit normal, query the collider shape here
        // or use cast_ray_and_get_normal if your Rapier version provides it.

        return Some(BulletHit {
            entity,
            distance: toi,
            position: hit_position,
            normal: Vec3::ZERO,
        });
    }

    None
}

fn fire_weapon(
    buttons: Res<ButtonInput<MouseButton>>,
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    rapier_context: ReadRapierContext,
    player_query: Query<Entity, With<Player>>,
    weapon_query: Query<&Weapon>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let transform = camera.single().unwrap();

    let origin = transform.translation();
    let direction = transform.forward();

    let player_entity = player_query.single().unwrap();
    let weapon = weapon_query.single().unwrap();

    if let Ok(ctx) = rapier_context.single() {
        if let Some(hit) = bullet_fire(&ctx, origin, *direction, weapon.range, player_entity) {
            println!(
                "{} hit {:?} for {} damage",
                weapon.name, hit.entity, weapon.damage
            );
        } else {
            println!("{} missed", weapon.name);
        }
    }
}

// #[derive(Asset, TypePath, Debug)]
// pub struct WeaponDefinition {
//     pub name: String,
//     pub damage: f32,
//     pub fire_rate: f32,
//     pub magazine_size: u32,

//     pub behavior: WeaponBehavior,
//     pub projectile: Option<Handle<ProjectileDefinition>>,
// }

// #[derive(Debug, Clone)]
// pub enum WeaponBehavior {
//     Melee {
//         range: f32,
//     },
//     Hitscan {
//         range: f32,
//     },
//     Projectile {
//         speed: f32,
//     },
// }

// #[derive(Component)]
// pub enum WeaponState {
//     Idle,
//     Firing,
//     Reloading,
//     Charging { progress: f32 },
// }

// fn weapon_attack_system(
//     time: Res<Time>,
//     definitions: Res<Assets<WeaponDefinition>>,
//     mut weapons: Query<(
//         &Weapon,
//         &mut Cooldown,
//         &Ammo,
//     )>,
// ) {
//     for (weapon, mut cooldown, ammo) in &mut weapons {
//         cooldown.timer.tick(time.delta());

//         if !cooldown.timer.is_finished() {
//             continue;
//         }

//         let Some(definition) = definitions.get(&weapon.definition) else {
//             continue;
//         };

//         // Decide what happens based on the weapon definition.
//         match &definition.behavior {
//             WeaponBehavior::Melee { .. } => {
//                 // ...
//             }

//             WeaponBehavior::Hitscan { .. } => {
//                 // ...
//             }

//             WeaponBehavior::Projectile { .. } => {
//                 // ...
//             }
//         }
//     }
// }

// enum WEAP {
//     NUM_WEAP_ANIMS = 32,
//     WEAP_ANIM_ADS_UP = 32,
//     WEAP_ANIM_ADS_DOWN = 31,
// }

// fn UpdateWeaponViewmodels() {

// }

// fn UpdateHandViewmodels() {

// }

// // Hit-scan
// fn BulletFire() {

// }

// fn fireweapon () {

// }
