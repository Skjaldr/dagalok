use bevy::prelude::*;

use crate::{characters::{animations::{get_animations, link_animations, play_animations}, spawn_char::spawn_character}, gamestate::GameState};

mod animations;
mod setup_char;
mod spawn_char;

pub struct CharactersPlugin;
impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LoadingCharacterMesh), (spawn_character, get_animations).chain())
            .add_systems(Update, (link_animations, play_animations).run_if(in_state(GameState::DoneLoading)));
            // .add_systems(Update, play_animations.run_if(in_state(GameState::DoneLoading)));
    }
}
