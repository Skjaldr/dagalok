use bevy::prelude::*;
use crate::{characters::{animations::{get_animations, link_animations, play_animations}, spawn_char::setup_character}, gamestate::GameState};

mod setup_char;
mod spawn_char;
mod animations;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LoadingCharacterMesh), setup_character)
            .add_systems(OnEnter(GameState::LoadingCharacterAnimations), (get_animations, link_animations).chain())
            .add_systems(Update, play_animations.run_if(in_state(GameState::DoneLoading)));
    }
}
