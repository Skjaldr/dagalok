use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

use crate::{asset_loader::AssetLoadingPlugin, characters::CharacterPlugin, gamestate::GameStatePlugin, setup::SetupPlugin, terrain::TerrainPlugin};

mod setup;
mod gamestate;
mod asset_loader;
mod characters;
mod terrain;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((
            DefaultPlugins,
            ThirdPersonCameraPlugin,
            GameStatePlugin,
            AssetLoadingPlugin,
            SetupPlugin,
            TerrainPlugin,
            CharacterPlugin,
        ))
        .run();
}
