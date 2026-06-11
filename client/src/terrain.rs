use bevy::prelude::*;

use crate::{gamestate::GameState};

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::LoadingTerrain), setup_terrain);
    }
}

pub fn setup_terrain(
    mut commands: Commands,
    // game_assets: Res<DkGameAssets>,
    asset_server: Res<AssetServer>,
    // gltf_handle: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let terrain_scene = GltfAssetLabel::Scene(0).from_asset("terrain/earth_floor.glb");
    commands.spawn(SceneRoot(asset_server.load(terrain_scene)));
    // let terrain_handle = asset_server.load(terrain_scene);

    // if let Some(terrain) = gltf_handle.get(&game_assets.terrain) {
    //     commands.spawn((
    //         SceneRoot(terrain.named_scenes["Scene"].clone()),
    //     ));
    // }

    next_state.set(GameState::LoadingCharacterMesh);
}
