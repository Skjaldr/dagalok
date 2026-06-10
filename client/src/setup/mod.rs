mod camera;
mod lights;

use bevy::prelude::*;

use crate::{gamestate::GameState, setup::{camera::setup_camera, lights::setup_lights}};

pub struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LoadingCamera), setup_camera)
            .add_systems(OnEnter(GameState::LoadingLights), setup_lights);
            // .add_systems(Update, handle_hotbar_clicks);

    }
}
