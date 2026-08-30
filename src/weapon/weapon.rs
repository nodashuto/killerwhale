use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy::light::ClusteredDecal;
use bevy::pbr::decal;

const BULLET_HOLE_TEXTURE: &str = "textures/decals/bullet_hole.png";
use std::collections::VecDeque;

// use crate::player::player::Player;
// use crate::player::player::PlayerCamera;

use crate::player::player::{Player, PlayerCamera, WeaponAnimationPlayer, WeaponWalkSway};

use bevy::animation::AnimationPlayer;
use bevy::animation::RepeatAnimation;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_plugin_loaded);
        app.add_systems(
            Update,
            (
                check_sprint,
                weapon_pose,
                fire_weapon,
                reload_weapon,
                update_reload,
                update_tracers,
                update_muzzle_flash,
                update_weapon_fire_timer,
            ),
        );
        app.init_resource::<BulletDecalManager>()
            .add_systems(Update, update_bullet_decals);
    }
}

/// prints plugin is loaded in console
fn print_plugin_loaded() {
    println!("weapon plugin is loaded");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FireMode {
    SemiAuto,
    FullAuto,
    Burst,
}

#[derive(Component)]
pub struct Weapon {
    pub definition: WeaponDefinition,

    pub ammo_in_magazine: u32,
    pub reserve_ammo: u32,
}

#[derive(Component, Debug, Clone)]
pub struct WeaponDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub model_path: &'static str,

    // index for blender animation
    pub idle_animation: usize,
    pub fire_animation: usize,
    pub reload_animation: usize,

    pub damage: f32,
    pub range: f32,

    pub hip_weapon_position: Vec3,
    pub ads_weapon_position: Vec3,
    pub sprint_weapon_position: Vec3,

    pub hip_weapon_rotation: Quat,
    pub ads_weapon_rotation: Quat,
    pub sprint_weapon_rotation: Quat,

    pub hip_muzzle_position: Vec3,
    pub ads_muzzle_position: Vec3,

    pub magazine_size: u32,
    pub reload_duration: f32,

    // Fire mode
    pub fire_mode: FireMode,
    pub fire_rate: f32,
}

#[derive(Component, Debug)]
pub struct WeaponState {
    pub is_reloading: bool,
    pub reload_timer: Timer,
    pub fire_timer: Timer,
    pub sprint_progress: f32,
    pub is_sprinting: bool,
}

impl WeaponState {
    pub fn new(fire_rate: f32) -> Self {
        let mut fire_timer = Timer::from_seconds(1.0 / fire_rate, TimerMode::Once);

        fire_timer.finish();

        Self {
            is_reloading: false,
            reload_timer: Timer::from_seconds(0.0, TimerMode::Once),
            fire_timer,
            sprint_progress: 0.0,
            is_sprinting: false,
        }
    }
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

pub fn check_sprint(keys: Res<ButtonInput<KeyCode>>, mut weapon_query: Query<&mut WeaponState>) {
    let sprinting = (keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight))
        && (keys.pressed(KeyCode::KeyW)
            || keys.pressed(KeyCode::KeyA)
            || keys.pressed(KeyCode::KeyS)
            || keys.pressed(KeyCode::KeyD));

    for mut state in &mut weapon_query {
        state.is_sprinting = sprinting;
    }
}

fn fire_weapon(
    buttons: Res<ButtonInput<MouseButton>>,
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    rapier_context: ReadRapierContext,
    player_query: Query<Entity, With<Player>>,

    mut decal_manager: ResMut<BulletDecalManager>,

    mut weapon_query: Query<(&mut Weapon, &mut WeaponState)>,

    muzzle_query: Query<&GlobalTransform, With<WeaponMuzzle>>,
    mut flash_query: Query<&mut MuzzleFlash>,
    mut light_query: Query<&mut Visibility, With<MuzzleFlashLight>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animation_query: Query<(&mut AnimationPlayer, &WeaponAnimationPlayer)>,
) {
    // let (mut weapon, weapon_state) = weapon_query.single_mut().unwrap();
    let Ok((mut weapon, mut weapon_state)) = weapon_query.single_mut() else {
        return;
    };

    // disable when sprint
    if weapon_state.is_sprinting {
        return;
    }

    let should_fire = match weapon.definition.fire_mode {
        FireMode::SemiAuto => buttons.just_pressed(MouseButton::Left),

        FireMode::FullAuto => buttons.pressed(MouseButton::Left),

        FireMode::Burst => buttons.just_pressed(MouseButton::Left),
    };

    if !should_fire {
        return;
    }

    let camera_transform = camera.single().unwrap();

    let player_entity = player_query.single().unwrap();

    // Can't fire while reloading
    if weapon_state.is_reloading {
        return;
    }

    // Fire-rate check
    if !weapon_state.fire_timer.is_finished() {
        return;
    }

    // Magazine empty
    if weapon.ammo_in_magazine == 0 {
        // println!("{}: magazine empty!", weapon.name);
        return;
    }

    let origin = camera_transform.translation();
    let direction = camera_transform.forward();

    let muzzle_transform = muzzle_query.single().unwrap();
    let muzzle_position = muzzle_transform.translation();

    let Ok(ctx) = rapier_context.single() else {
        return;
    };

    let end_position = if let Some(hit) = bullet_fire(
        &ctx,
        origin,
        *direction,
        weapon.definition.range,
        player_entity,
    ) {
        println!(
            "{} hit {:?} for {} damage",
            weapon.definition.name, hit.entity, weapon.definition.damage
        );

        // spawn decal
        spawn_bullet_hole_test(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut decal_manager,
            hit.position + hit.normal * 0.05,
        );

        hit.position
    } else {
        println!("{} missed", weapon.definition.name);

        origin + *direction * weapon.definition.range
    };

    // Consume one bullet
    weapon.ammo_in_magazine -= 1;

    println!(
        "{} ammo: {}/{} | reserve: {}",
        weapon.definition.name,
        weapon.ammo_in_magazine,
        weapon.definition.magazine_size,
        weapon.reserve_ammo
    );

    // Reset fire timer
    weapon_state.fire_timer.reset();

    for (mut player, animations) in &mut animation_query {
        player
            .play(animations.fire)
            .set_speed(4.0)
            // .set_repeat(RepeatAnimation::Count(1))
            .set_repeat(RepeatAnimation::Never)
            .replay();
        //.seek_to(0.0);
    }

    // https://docs.rs/bevy/latest/bevy/animation/struct.ActiveAnimation.html#implementations

    // https://docs.rs/bevy/latest/bevy/animation/enum.RepeatAnimation.html

    // --------------------------------------------------------
    // MUZZLE FLASH
    // --------------------------------------------------------

    if let Ok(mut flash) = flash_query.single_mut() {
        flash.timer.reset();
    }

    for mut visibility in &mut light_query {
        *visibility = Visibility::Visible;
    }
    // --------------------------------------------------------
    // TRACER
    // --------------------------------------------------------

    let direction = (end_position - muzzle_position).normalize();
    let tracer_length = 2.75;
    let tracer_position = muzzle_position + direction * (tracer_length * 0.5);

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(0.008, 0.008, tracer_length)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.75, 0.3),
            emissive: LinearRgba::new(1.0, 0.60, 0.0, 1.0),
            ..default()
        })),
        Transform::from_translation(tracer_position).looking_to(direction, Vec3::Y),
        BulletTracer::new(muzzle_position, end_position, 600.0),
    ));
}

// // for ADS animation
// fn weapon_ads(
//     time: Res<Time>,
//     buttons: Res<ButtonInput<MouseButton>>,
//     mut weapon_query: Query<(&mut Transform, &Weapon, &mut WeaponAds), Without<WeaponMuzzle>>,
//     mut muzzle_query: Query<(&mut Transform, &mut WeaponMuzzle), With<WeaponMuzzle>>,
// ) {
//     let target = if buttons.pressed(MouseButton::Right) {
//         1.0
//     } else {
//         0.0
//     };

//     let ads_speed = 8.0;

//     // -------------------------
//     // Weapon
//     // -------------------------

//     for (mut transform, weapon, mut ads) in &mut weapon_query {
//         ads.progress = move_toward(ads.progress, target, ads_speed * time.delta_secs());

//         let t = ads.progress;
//         let t = t * t * (3.0 - 2.0 * t);

//         transform.translation = weapon
//             .definition
//             .hip_weapon_position
//             .lerp(weapon.definition.ads_weapon_position, t);
//     }

//     // -------------------------
//     // Muzzle
//     // -------------------------

//     for (mut transform, mut muzzle) in &mut muzzle_query {
//         muzzle.progress = move_toward(muzzle.progress, target, ads_speed * time.delta_secs());

//         let t = muzzle.progress;
//         let t = t * t * (3.0 - 2.0 * t);

//         transform.translation = muzzle.hip_position.lerp(muzzle.ads_position, t);
//     }
// }

// /// new ADS compatible with sway anim
// fn weapon_ads(
//     time: Res<Time>,
//     buttons: Res<ButtonInput<MouseButton>>,
//     mut weapon_query: Query<(&Weapon, &mut WeaponAds, &mut WeaponWalkSway), Without<WeaponMuzzle>>,
//     mut muzzle_query: Query<(&mut Transform, &mut WeaponMuzzle), With<WeaponMuzzle>>,
// ) {
//     let target = if buttons.pressed(MouseButton::Right) {
//         1.0
//     } else {
//         0.0
//     };

//     let ads_speed = 8.0;
//     let dt = time.delta_secs();

//     for (weapon, mut ads, mut sway) in &mut weapon_query {
//         ads.progress = move_toward(ads.progress, target, ads_speed * dt);

//         let t = ads.progress;
//         let t = t * t * (3.0 - 2.0 * t);

//         sway.base_translation = weapon
//             .definition
//             .hip_weapon_position
//             .lerp(weapon.definition.ads_weapon_position, t);

//         // If your Weapon definition has rotations:
//         // sway.base_rotation = weapon
//         //     .definition
//         //     .hip_weapon_rotation
//         //     .slerp(weapon.definition.ads_weapon_rotation, t);
//     }

//     for (mut transform, mut muzzle) in &mut muzzle_query {
//         muzzle.progress = move_toward(muzzle.progress, target, ads_speed * dt);

//         let t = muzzle.progress;
//         let t = t * t * (3.0 - 2.0 * t);

//         transform.translation = muzzle.hip_position.lerp(muzzle.ads_position, t);
//     }
// }

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

fn update_muzzle_flash(time: Res<Time>, mut query: Query<(&mut MuzzleFlash, &mut Visibility)>) {
    for (mut flash, mut visibility) in &mut query {
        if !flash.timer.is_finished() {
            flash.timer.tick(time.delta());

            *visibility = Visibility::Visible;
        //*visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn reload_weapon(
    keys: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<(&Weapon, &mut WeaponState)>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    let Ok((weapon, mut state)) = weapon_query.single_mut() else {
        return;
    };

    if state.is_reloading {
        return;
    }

    if weapon.ammo_in_magazine >= weapon.definition.magazine_size {
        return;
    }

    if weapon.reserve_ammo == 0 {
        return;
    }

    state.is_reloading = true;

    state.reload_timer = Timer::from_seconds(weapon.definition.reload_duration, TimerMode::Once);

    println!(
        "{}: reloading ({:.1}s)",
        weapon.definition.name, weapon.definition.reload_duration
    );
}

fn update_reload(time: Res<Time>, mut weapon_query: Query<(&mut Weapon, &mut WeaponState)>) {
    let Ok((mut weapon, mut state)) = weapon_query.single_mut() else {
        return;
    };

    if !state.is_reloading {
        return;
    }

    state.reload_timer.tick(time.delta());

    if state.reload_timer.just_finished() {
        let needed = weapon.definition.magazine_size - weapon.ammo_in_magazine;

        let amount = needed.min(weapon.reserve_ammo);

        weapon.ammo_in_magazine += amount;
        weapon.reserve_ammo -= amount;

        state.is_reloading = false;

        println!(
            "{}: reload complete, {}/{} | reserve: {}",
            weapon.definition.name,
            weapon.ammo_in_magazine,
            weapon.definition.magazine_size,
            weapon.reserve_ammo
        );
    }
}

fn update_weapon_fire_timer(time: Res<Time>, mut weapon_query: Query<&mut WeaponState>) {
    let Ok(mut state) = weapon_query.single_mut() else {
        return;
    };

    state.fire_timer.tick(time.delta());
}

// pub fn weapon_sprint(
//     time: Res<Time>,
//     mut weapon_query: Query<
//         (
//             &Weapon,
//             &mut WeaponState,
//             &mut WeaponAds,
//             &mut WeaponWalkSway,
//         ),
//         Without<WeaponMuzzle>,
//     >,
// ) {
//     let dt = time.delta_secs();
//     let sprint_speed = 8.0;

//     for (weapon, mut state, mut ads, mut sway) in &mut weapon_query {
//         // ---------------------------------
//         // Sprint
//         // ---------------------------------

//         let sprint_target = if state.is_sprinting {
//             1.0
//         } else {
//             0.0
//         };

//         state.sprint_progress = move_toward(
//             state.sprint_progress,
//             sprint_target,
//             sprint_speed * dt,
//         );

//         let sprint_t = state.sprint_progress;
//         let sprint_t = sprint_t * sprint_t * (3.0 - 2.0 * sprint_t);

//         // ---------------------------------
//         // Disable ADS while sprinting
//         // ---------------------------------

//         if state.is_sprinting {
//             ads.progress = move_toward(
//                 ads.progress,
//                 0.0,
//                 12.0 * dt,
//             );
//         }

//         let ads_t = ads.progress;
//         let ads_t = ads_t * ads_t * (3.0 - 2.0 * ads_t);

//         // ---------------------------------
//         // HIP -> ADS
//         // ---------------------------------

//         let normal_translation = weapon
//             .definition
//             .hip_weapon_position
//             .lerp(
//                 weapon.definition.ads_weapon_position,
//                 ads_t,
//             );

//         let normal_rotation = weapon
//             .definition
//             .hip_weapon_rotation
//             .slerp(
//                 weapon.definition.ads_weapon_rotation,
//                 ads_t,
//             );

//         // ---------------------------------
//         // HIP/ADS -> SPRINT
//         // ---------------------------------

//         sway.base_translation = normal_translation.lerp(
//             weapon.definition.sprint_weapon_position,
//             sprint_t,
//         );

//         sway.base_rotation = normal_rotation.slerp(
//             weapon.definition.sprint_weapon_rotation,
//             sprint_t,
//         );
//     }
// }

pub fn weapon_pose(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut weapon_query: Query<
        (
            &Weapon,
            &mut WeaponState,
            &mut WeaponAds,
            &mut WeaponWalkSway,
        ),
        Without<WeaponMuzzle>,
    >,
    mut muzzle_query: Query<(&mut Transform, &mut WeaponMuzzle), With<WeaponMuzzle>>,
) {
    let dt = time.delta_secs();

    for (weapon, mut state, mut ads, mut sway) in &mut weapon_query {
        // -------------------------
        // ADS
        // -------------------------

        let wants_ads = buttons.pressed(MouseButton::Right);

        // RMB cancels sprint.
        if wants_ads {
            state.is_sprinting = false;
        }

        let ads_target = if wants_ads { 1.0 } else { 0.0 };

        ads.progress = move_toward(ads.progress, ads_target, 8.0 * dt);

        let ads_t = ads.progress;
        let ads_t = ads_t * ads_t * (3.0 - 2.0 * ads_t);

        // -------------------------
        // Sprint
        // -------------------------

        let sprint_target = if state.is_sprinting { 1.0 } else { 0.0 };

        state.sprint_progress = move_toward(state.sprint_progress, sprint_target, 8.0 * dt);

        let sprint_t = state.sprint_progress;
        let sprint_t = sprint_t * sprint_t * (3.0 - 2.0 * sprint_t);

        // -------------------------
        // HIP -> ADS
        // -------------------------

        let normal_translation = weapon
            .definition
            .hip_weapon_position
            .lerp(weapon.definition.ads_weapon_position, ads_t);

        let normal_rotation = weapon
            .definition
            .hip_weapon_rotation
            .slerp(weapon.definition.ads_weapon_rotation, ads_t);

        // -------------------------
        // HIP/ADS -> SPRINT
        // -------------------------

        sway.base_translation =
            normal_translation.lerp(weapon.definition.sprint_weapon_position, sprint_t);

        sway.base_rotation =
            normal_rotation.slerp(weapon.definition.sprint_weapon_rotation, sprint_t);
    }

    let target = if buttons.pressed(MouseButton::Right) {
        1.0
    } else {
        0.0
    };

    let ads_speed = 2.0;

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

#[derive(Component)]
pub struct BulletHoleDecal {
    pub lifetime: Timer,
}

#[derive(Resource, Default)]
pub struct BulletDecalManager {
    pub decals: VecDeque<Entity>,
}

const MAX_BULLET_DECALS: usize = 200;
const BULLET_DECAL_LIFETIME: f32 = 10.0;

// fn spawn_bullet_hole_decal(
//     commands: &mut Commands,
//     asset_server: &AssetServer,
//     position: Vec3,
//     normal: Vec3,
// ) {
//     let position = position + normal * 0.005;

//     let rotation = Quat::from_rotation_arc(Vec3::Z, -normal);

//     let temp = 4.0;

//     commands.spawn((
//         ClusteredDecal {
//             base_color_texture: Some(asset_server.load(BULLET_HOLE_TEXTURE)),
//             ..default()
//         },
//         Transform {
//             translation: position,
//             rotation,
//             scale: Vec3::new(temp, temp, temp),
//         },
//         BulletHoleDecal,
//     ));
// }

fn spawn_bullet_hole_test(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    manager: &mut BulletDecalManager,
    position: Vec3,
) {
    // Remove the oldest decal if we reached the limit.
    if manager.decals.len() >= MAX_BULLET_DECALS {
        if let Some(oldest) = manager.decals.pop_front() {
            commands.entity(oldest).despawn();
        }
    }

    let entity = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.05))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                emissive: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
                ..default()
            })),
            Transform::from_translation(position),
            BulletHoleDecal {
                lifetime: Timer::from_seconds(BULLET_DECAL_LIFETIME, TimerMode::Once),
            },
        ))
        .id();

    manager.decals.push_back(entity);
}

fn update_bullet_decals(
    time: Res<Time>,
    mut commands: Commands,
    mut manager: ResMut<BulletDecalManager>,
    mut decals: Query<&mut BulletHoleDecal>,
) {
    let mut expired = Vec::new();

    for &entity in &manager.decals {
        let Ok(mut decal) = decals.get_mut(entity) else {
            expired.push(entity);
            continue;
        };

        decal.lifetime.tick(time.delta());

        if decal.lifetime.is_finished() {
            expired.push(entity);
        }
    }

    for entity in expired {
        manager.decals.retain(|&e| e != entity);
        commands.entity(entity).despawn();
    }
}

// pub fn weapon_pose(
//     time: Res<Time>,
//     buttons: Res<ButtonInput<MouseButton>>,
//     mut weapon_query: Query<
//         (
//             &Weapon,
//             &mut WeaponState,
//             &mut WeaponAds,
//             &mut WeaponWalkSway,
//         ),
//         Without<WeaponMuzzle>,
//     >,
// ) {
//     let dt = time.delta_secs();

//     for (weapon, mut state, mut ads, mut sway) in &mut weapon_query {
//         // -------------------------
//         // ADS
//         // -------------------------

//         let ads_target = if buttons.pressed(MouseButton::Right)
//             && !state.is_sprinting
//         {
//             1.0
//         } else {
//             0.0
//         };

//         ads.progress = move_toward(
//             ads.progress,
//             ads_target,
//             8.0 * dt,
//         );

//         let ads_t = ads.progress;
//         let ads_t = ads_t * ads_t * (3.0 - 2.0 * ads_t);

//         // -------------------------
//         // Sprint
//         // -------------------------

//         let sprint_target = if state.is_sprinting {
//             1.0
//         } else {
//             0.0
//         };

//         state.sprint_progress = move_toward(
//             state.sprint_progress,
//             sprint_target,
//             8.0 * dt,
//         );

//         let sprint_t = state.sprint_progress;
//         let sprint_t =
//             sprint_t * sprint_t * (3.0 - 2.0 * sprint_t);

//         // -------------------------
//         // HIP -> ADS
//         // -------------------------

//         let normal_translation = weapon
//             .definition
//             .hip_weapon_position
//             .lerp(
//                 weapon.definition.ads_weapon_position,
//                 ads_t,
//             );

//         let normal_rotation = weapon
//             .definition
//             .hip_weapon_rotation
//             .slerp(
//                 weapon.definition.ads_weapon_rotation,
//                 ads_t,
//             );

//         // -------------------------
//         // HIP/ADS -> SPRINT
//         // -------------------------

//         sway.base_translation = normal_translation.lerp(
//             weapon.definition.sprint_weapon_position,
//             sprint_t,
//         );

//         sway.base_rotation = normal_rotation.slerp(
//             weapon.definition.sprint_weapon_rotation,
//             sprint_t,
//         );
//     }
// }

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
