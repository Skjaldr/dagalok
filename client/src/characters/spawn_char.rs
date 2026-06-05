use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

use crate::{asset_loader::DkGameAssets, characters::setup_char::{CharacterBundle, Health, Speed}, gamestate::GameState};

pub fn setup_character(
    mut commands: Commands,
    game_assets: Res<DkGameAssets>,
    gltf_handle: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(character_mesh) = gltf_handle.get(&game_assets.character_mesh) {
        commands.spawn((
            SceneRoot(character_mesh.named_scenes["Scene"].clone()),
            Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ThirdPersonCameraTarget,
            CharacterBundle::new(Vec3::new(0.0, 0.0, 0.0), Health { max: 100.0, current: 100.0 }, Speed(1.0))
        ));
    }
    next_state.set(GameState::LoadingCharacterAnimations);
}
