
// #[derive(Resource, Deref)]
// struct SoundEffect {
//     handle: Handle<AudioSource>,
// }

// // We can setup the logic for how to load our assets in the `FromWorld` trait.
// // This code is called via `init_resource`.
// impl FromWorld for SoundEffect {
//     fn from_world(world: &mut World) -> Self {
//         let asset_server = world.resource::<AssetServer>();
//         SoundEffect {
//             handle: asset_server.load("sounds/glock_single_shot_modifiy.ogg"),
//         }
//     }
// }
