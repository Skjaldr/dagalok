use bevy::prelude::*;

use crate::{gamestate::GameState::{self, DoneLoading}, player::input::character_movement};
pub mod input;
pub mod setup_player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, character_movement);
    }
}
