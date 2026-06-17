use bevy::prelude::*;
<<<<<<< HEAD
use crate::{characters::{animations::{get_animations, update_animations}, spawn_char::spawn_player_character}, gamestate::GameState};
=======
use crate::{characters::{animations::{get_animations, play_animations, update_animations}, spawn_char::spawn_player_character}, gamestate::GameState};
>>>>>>> 2385666 (changes to animation system)
// use crate::characters::animations::link_animations;

pub mod setup_char;
mod spawn_char;
mod animations;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LoadingCharacterMesh), spawn_player_character)
            .add_systems(OnEnter(GameState::LoadingCharacterAnimations), get_animations)
            .add_systems(Update, update_animations);
    }
}
