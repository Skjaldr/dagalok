use bevy::prelude::*;
mod connection;

use crate::{module_bindings::{self, DbConnection}, networking::connection::{connect_to_server, process_server_messages}};

#[derive(Resource)]
pub struct SpacetimeConnection {
    pub conn: DbConnection,
}

pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, connect_to_server)
            .add_systems(Update, process_server_messages);
    }
}
