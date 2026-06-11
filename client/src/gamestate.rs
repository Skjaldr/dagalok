use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>();
    }
}

#[derive(States, Copy, Hash, Eq, PartialEq, Debug, Clone, Default)]
pub enum GameState {
    #[default]
    LoadingCamera,
    LoadingLights,
    LoadingTerrain,
    LoadingCharacterMesh,
    LoadingCharacterAnimations,
    DoneLoading
}
