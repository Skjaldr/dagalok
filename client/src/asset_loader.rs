// use bevy::prelude::*;
// use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState}};

// use crate::gamestate::GameState;

// pub struct AssetLoadingPlugin;
// impl Plugin for AssetLoadingPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .add_loading_state(LoadingState::new(GameState::LoadingAssets)
//                 .continue_to_state(GameState::LoadingCamera)
//                 .load_collection::<DkGameAssets>()
//             );
//     }
// }

// #[derive(AssetCollection, Resource)]
// pub struct DkGameAssets {
//     #[asset( path = "models/target.glb")]
//     pub character_mesh: Handle<Gltf>,
//     #[asset( path = "terrain/earth_floor.glb")]
//     pub terrain: Handle<Gltf>,
//     #[asset( path = "source_3.glb")]
//     pub skeleton: Handle<Gltf>,
// }
