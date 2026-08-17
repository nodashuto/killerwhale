use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::player::Player;
use crate::player::player::PlayerCamera;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_plugin_loaded);
        app.add_systems(
            Update,
            (weapon_ads, fire_weapon, update_tracers, update_muzzle_flash),
        );
    }
}

/// prints plugin is loaded in console
fn print_plugin_loaded() {
    println!("weapon plugin is loaded");
}

// /// stores weapon name, damage
// #[derive(Component, Debug)]
// pub struct Weapon {
//     // the name never changes
//     pub name: &'static str,
//     pub damage: f32,
//     pub range: f32,
// }

#[derive(Component, Debug)]
pub struct Weapon {
    pub name: &'static str,
    pub damage: f32,
    pub range: f32,

    pub hip_position: Vec3,
    pub ads_position: Vec3,
}

#[derive(Component, Debug, Default)]
pub struct WeaponAds {
    /// 0.0 = hip
    /// 1.0 = fully ADS
    pub progress: f32,
}

#[derive(Component, Debug)]
pub struct WeaponMuzzle {
    pub hip_position: Vec3,
    pub ads_position: Vec3,

    /// 0.0 = hip
    /// 1.0 = ADS
    pub progress: f32,
}

#[derive(Component)]
pub struct MuzzleFlashLight;

#[derive(Component)]
pub struct MuzzleFlash {
    pub timer: Timer,
}

#[derive(Debug)]
pub struct BulletHit {
    pub entity: Entity,
    pub distance: f32,
    pub position: Vec3,
    pub normal: Vec3,
}

#[derive(Component, Debug)]
pub struct BulletTracer {
    pub start_position: Vec3,
    pub end_position: Vec3,
    pub direction: Vec3,
    pub speed: f32,
}

// impl BulletTracer {
//     pub fn new(start: Vec3, end: Vec3, speed: f32) -> Self {
//         let direction = (end - start).normalize_or_zero();

//         Self {
//             start_position: start,
//             end_position: end,
//             direction,
//             speed,
//         }
//     }
// }

impl BulletTracer {
    pub fn new(start_position: Vec3, end_position: Vec3, speed: f32) -> Self {
        let direction = (end_position - start_position).normalize_or_zero();

        Self {
            start_position,
            end_position,
            direction,
            speed,
        }
    }
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
    muzzle_query: Query<&GlobalTransform, With<WeaponMuzzle>>,
    mut flash_query: Query<&mut MuzzleFlash>,
    mut light_query: Query<&mut Visibility, With<MuzzleFlashLight>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let camera_transform = camera.single().unwrap();
    let weapon = weapon_query.single().unwrap();
    let player_entity = player_query.single().unwrap();

    let origin = camera_transform.translation();
    let direction = camera_transform.forward();

    let muzzle_transform = muzzle_query.single().unwrap();
    let muzzle_position = muzzle_transform.translation();

    let Ok(ctx) = rapier_context.single() else {
        return;
    };

    let end_position =
        if let Some(hit) = bullet_fire(&ctx, origin, *direction, weapon.range, player_entity) {
            println!(
                "{} hit {:?} for {} damage",
                weapon.name, hit.entity, weapon.damage
            );

            hit.position
        } else {
            println!("{} missed", weapon.name);

            origin + *direction * weapon.range
        };

    // --------------------------------------------------------
    // MUZZLE FLASH
    // --------------------------------------------------------

    if let Ok(mut flash) = flash_query.single_mut() {
        flash.timer.reset();
    }

    for mut visibility in &mut light_query {
        *visibility = Visibility::Visible;
    }

    // let color = Srgba::rgb(0.5, 0.5, 0.5);

    // let linear_color: LinearRgba = color.into();

    // --------------------------------------------------------
    // TRACER
    // --------------------------------------------------------

    // // Tracer starts at the actual gun muzzle.
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(0.01, 0.01, 1.0)))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         //base_color: Color::srgb( 110.0 / 255.0 , 56.0 / 255.0,  200.0/255.0),
    //         base_color: Color::srgb(1.0, 0.75, 0.0),
    //         emissive: LinearRgba::new(1.0, 0.75, 0.0, 1000.0),
    //         ..default()
    //     })),
    //     Transform::from_translation(muzzle_position),
    //     BulletTracer::new(muzzle_position, end_position, 100.0),
    // ));

    let direction = (end_position - muzzle_position).normalize();

    let tracer_length = 1.0;

    let tracer_position = muzzle_position + direction * (tracer_length * 0.5);

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(0.01, 0.01, tracer_length)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.75, 0.0),
            emissive: LinearRgba::new(1.0, 0.75, 0.0, 1000.0),
            ..default()
        })),
        Transform::from_translation(tracer_position).looking_to(direction, Vec3::Y),
        BulletTracer::new(muzzle_position, end_position, 110.0),
    ));
}

// for ADS animation
fn weapon_ads(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut weapon_query: Query<(&mut Transform, &Weapon, &mut WeaponAds), Without<WeaponMuzzle>>,
    mut muzzle_query: Query<(&mut Transform, &mut WeaponMuzzle), With<WeaponMuzzle>>,
) {
    let target = if buttons.pressed(MouseButton::Right) {
        1.0
    } else {
        0.0
    };

    let ads_speed = 8.0;

    // -------------------------
    // Weapon
    // -------------------------

    for (mut transform, weapon, mut ads) in &mut weapon_query {
        ads.progress = move_toward(ads.progress, target, ads_speed * time.delta_secs());

        let t = ads.progress;
        let t = t * t * (3.0 - 2.0 * t);

        transform.translation = weapon.hip_position.lerp(weapon.ads_position, t);
    }

    // -------------------------
    // Muzzle
    // -------------------------

    for (mut transform, mut muzzle) in &mut muzzle_query {
        muzzle.progress = move_toward(muzzle.progress, target, ads_speed * time.delta_secs());

        let t = muzzle.progress;
        let t = t * t * (3.0 - 2.0 * t);

        transform.translation = muzzle.hip_position.lerp(muzzle.ads_position, t);
    }
}

fn move_toward(current: f32, target: f32, max_delta: f32) -> f32 {
    let delta = target - current;

    if delta.abs() <= max_delta {
        target
    } else {
        current + delta.signum() * max_delta
    }
}

// fn update_tracers(
//     mut commands: Commands,
//     mut query: Query<(Entity, &BulletTracer, &mut Transform)>,
//     time: Res<Time>,
// ) {
//     for (entity, tracer, mut transform) in &mut query {
//         // Move forward like a projectile.
//         transform.translation += tracer.direction * tracer.speed * time.delta_secs();

//         // Keep the tracer pointing in the shooting direction.
//         if tracer.direction.length_squared() > 0.0 {
//             transform.look_to(tracer.direction, Vec3::Y);
//         }

//         // Check whether we've passed the destination.
//         let distance_to_end = transform.translation.distance(tracer.end_position);

//         let distance_from_start = transform.translation.distance(tracer.start_position);

//         let total_distance = tracer.start_position.distance(tracer.end_position);

//         if distance_to_end < tracer.speed * time.delta_secs()
//             || distance_from_start >= total_distance
//         {
//             commands.entity(entity).despawn();
//         }
//     }
// }

fn update_tracers(
    mut commands: Commands,
    mut query: Query<(Entity, &BulletTracer, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, tracer, mut transform) in &mut query {
        let direction = tracer.direction.normalize_or_zero();

        // Move forward.
        transform.translation += direction * tracer.speed * time.delta_secs();

        // Point the tracer in the same direction.
        if direction.length_squared() > 0.0 {
            transform.look_to(direction, Vec3::Y);
        }

        // Despawn when close enough to the destination.
        let distance_to_end = transform.translation.distance(tracer.end_position);

        if distance_to_end < 1.0 {
            commands.entity(entity).despawn();
        }
    }
}

// fn update_tracers(
//     mut commands: Commands,
//     mut query: Query<(Entity, &BulletTracer, &mut Transform)>,
//     time: Res<Time>,
// ) {
//     for (entity, tracer, mut transform) in &mut query {
//         // Move forward like a projectile.
//         transform.translation += tracer.direction * tracer.speed * time.delta_secs();

//         // Keep the tracer pointing in the shooting direction.
//         if tracer.direction.length_squared() > 0.0 {
//             transform.look_to(tracer.direction, Vec3::Y);
//         }

//         // Check whether we've passed the destination.
//         let distance_to_end = transform.translation.distance(tracer.end_position);

//         let distance_from_start = transform.translation.distance(tracer.start_position);

//         let total_distance = tracer.start_position.distance(tracer.end_position);

//         if distance_to_end < 2.0 || distance_from_start >= total_distance {
//             commands.entity(entity).despawn();
//         }
//     }
// }

fn update_muzzle_flash(time: Res<Time>, mut query: Query<(&mut MuzzleFlash, &mut Visibility)>) {
    for (mut flash, mut visibility) in &mut query {
        if !flash.timer.is_finished() {
            flash.timer.tick(time.delta());

            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
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
