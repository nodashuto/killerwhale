use bevy::prelude::*;


pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print_plugin_loaded);        
    }
}



fn print_plugin_loaded() {
    println!("weapon plugin is loaded");
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
