use avian3d::{collision::collider::Collider, debug_render::DebugRender, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

use crate::{characters::setup_char::{Target, Targettable}, gamestate::GameState, ui::systems::get_target};

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
        Pickable::default(),
        Targettable,

    )).with_children(|parent| {
        parent.spawn((
            Collider::cuboid(81.0, 0.0, 81.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
            DebugRender::default().with_collider_color(Color::Srgba(Srgba::RED)),
        ));
    }).observe(get_target::<Pointer<Click>>());


    next_state.set(GameState::LoadingCharacterMesh);
}
