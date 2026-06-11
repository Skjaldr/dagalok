use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

use crate::{characters::setup_char::{CharacterBundle, Health, Speed}, gamestate::GameState, player::setup_player::PlayerName};
use crate::player::setup_player::Player;


pub fn spawn_player_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // game_assets: Res<DkGameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // if let Some(character_mesh) = gltf_handle.get(&game_assets.character_mesh) {
    let character = GltfAssetLabel::Scene(0).from_asset("models/target.glb");
    let scene_handle = asset_server.load(character);
        commands.spawn((
            SceneRoot(scene_handle.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ThirdPersonCameraTarget,
            CharacterBundle::new(Vec3::new(0.0, 0.0, 0.0), Health { max: 100.0, current: 100.0 }, Speed(1.0), false),

            // Adding a Player Component marker until defining difference between NPCs and player Characaters
            Player,
            PlayerName("Joe".to_string()),


            // THE IDEA HERE IS TO ADD AN OBSERVER THAT WILL CYCLE THROUGH ALL OF THE CHILDREN OF THE ENTITY AND PICK OUT
            // THE BITS THAT WILL BE USED LATER ON SUCH AS ANIMATIONS AND MESH.  This will allow for the assets to be loaded as needed
            // instead of all at once on startup.
        ));

    // }
    next_state.set(GameState::LoadingCharacterAnimations);
}
