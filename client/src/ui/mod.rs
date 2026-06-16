use bevy::prelude::*;

use crate::{gamestate::GameState, ui::systems::{
    // get_targeted_player,
    local_player_name_bar, local_player_target_bar, spawn_health_bar, update_health_bar, update_target_bar}};
pub mod systems;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::DoneLoading), (spawn_health_bar, local_player_name_bar, local_player_target_bar))
            .add_systems(Update, update_health_bar)
            .add_systems(Update, update_target_bar.run_if(in_state(GameState::DoneLoading)));

    }
}
