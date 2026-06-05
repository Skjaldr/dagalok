use bevy::prelude::*;

use crate::{asset_loader::AssetLoadingPlugin, gamestate::GameStatePlugin, setup::SetupPlugin, terrain::TerrainPlugin};

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
            GameStatePlugin,
            AssetLoadingPlugin,
            SetupPlugin,
            TerrainPlugin,

        ))
        .run();
}
