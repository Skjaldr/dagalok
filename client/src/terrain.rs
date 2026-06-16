use avian3d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

use crate::{gamestate::GameState, ui::systems::get_targeted_player};

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
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
    commands.spawn((
        SceneRoot(asset_server.load(terrain_scene)),

    )).with_children(|parent| {
        parent.spawn((
            RigidBody::Static,
            Collider::cuboid(81.0, 0.0, 81.0),
            Pickable::default(),
        ));
    })
    .observe(get_targeted_player::<Pointer<Click>>());


    next_state.set(GameState::LoadingCharacterMesh);
}
