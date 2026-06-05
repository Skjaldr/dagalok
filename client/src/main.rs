use bevy::prelude::*;

use crate::{gamestate::GameStatePlugin, setup::SetupPlugin};

mod setup;
mod gamestate;
mod asset_loader;
mod characters;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((
            DefaultPlugins,
            GameStatePlugin,
            SetupPlugin,

        ))
        .run();
}
