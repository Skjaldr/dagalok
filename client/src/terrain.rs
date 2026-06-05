use bevy::prelude::*;

use crate::{asset_loader::DkGameAssets, gamestate::GameState};

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LoadingTerrain), setup_terrain);
    }
}

pub fn setup_terrain(
    mut commands: Commands,
    game_assets: Res<DkGameAssets>,
    gltf_handle: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(terrain) = gltf_handle.get(&game_assets.terrain) {
        commands.spawn((
            SceneRoot(terrain.named_scenes["Scene"].clone()),
        ));
    }

    next_state.set(GameState::LoadingCharacterMesh);
}
