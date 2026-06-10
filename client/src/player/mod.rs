use bevy::prelude::*;

use crate::{gamestate::GameState::{self, DoneLoading}, player::{input::character_movement, ui_hud::{spawn_health_bar, update_health_bar}}};

pub mod input;
pub mod setup_player;
pub mod ui_hud;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(DoneLoading), spawn_health_bar)
            .add_systems(Update, (character_movement, update_health_bar));
    }
}
