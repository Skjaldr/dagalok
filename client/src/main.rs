use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

use crate::{characters::CharacterPlugin, gamestate::GameStatePlugin, npc::NonPlayerCharacterPlugin, player::PlayerPlugin, setup::SetupPlugin, terrain::TerrainPlugin, ui::UiPlugin};

mod setup;
mod gamestate;
mod asset_loader;
mod characters;
mod terrain;
mod player;
mod ui;
mod npc;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            ThirdPersonCameraPlugin,
            GameStatePlugin,
            SetupPlugin,
            TerrainPlugin,
            CharacterPlugin,
            PlayerPlugin,
            MeshPickingPlugin,
            PhysicsPickingPlugin,
            PhysicsDebugPlugin::default(),
            UiPlugin,
            NonPlayerCharacterPlugin,
        ))
        .run();
}
