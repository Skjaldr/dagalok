use bevy::prelude::*;

use crate::{gamestate::GameState, npc::spawn::spawn_npc};

mod spawn;
mod npc_setup;

pub struct NonPlayerCharacterPlugin;
impl Plugin for NonPlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LoadingCharacterMesh), spawn_npc);
    }
}
