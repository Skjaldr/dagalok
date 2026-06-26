use crate::module_bindings::{self, DbConnection, PlayerTableAccess, playerQueryTableAccess};

use bevy::prelude::*;
use spacetimedb_sdk::{DbConnectionBuilder, DbContext, Table};
use std::path::PathBuf;

const SPACETIMEDB_URI: &str = "http://127.0.0.1:3000";
const DB_NAME: &str =  "dagalok";
const TOKEN: &str = "spacetimedb_token";

use super::SpacetimeConnection;


fn token_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    Some(exe.parent()?.join(TOKEN))
}

fn load_token() -> Option<String> {
    let path = token_path()?;
    let contents = std::fs::read_to_string(&path).ok()?;
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn save_token(token: &str) -> std::io::Result<()> {
    let path = token_path().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Could not determine executable path")
    })?;
    std::fs::write(path, token)
}


pub fn connect_to_server(
    mut commands: Commands,
) {
    let token = load_token();

    // let conn = DbConnectionBuilder::build();
    let conn = DbConnection::builder()
        .with_uri(SPACETIMEDB_URI)
        .with_database_name(DB_NAME)
        .with_token(token)
        .on_connect(|ctx, _identity, token| {
            if let Err(e) = save_token(token) {
                error!("Failed to save token: {e}")
            }
            info!("Connected to server");
            ctx.subscription_builder()
                .on_applied(|ctx| {
                    if let Some(identity) = ctx.try_identity() {
                        if let Some(player) = ctx.db.player().identity().find(&identity) {
                            info!("playing as: {}", player.username);
                            return;
                        }
                    }
                    info!("Player subscription applied");
                })
                .on_error(|_ctx, err| {
                    error!("Subscription error: {err}");
                })
                .add_query(|q| q.from.player())
                .subscribe();
        })
        .on_connect_error(|_ctx, err| {
            error!("Server connection error: {err}");
        })
        .on_disconnect(|_ctx, err| {
            if let Some(e) = err {
                warn!("Disconnected from server with error: {e}");
            } else {
                info!("Disconnected from server");
            }
        })
        .build();

    match conn {
        Ok(conn) => {
            info!("server connection initiated");
            commands.insert_resource(SpacetimeConnection { conn });
        }
        Err(e) => {
            error!("Failed to initiate server connection: {e}");
        }
    }
}

pub fn process_server_messages(
    connection: Res<SpacetimeConnection>
) {
    if let Err(e) = connection.conn.frame_tick() {
        error!("server frame_tick error: {e}");
    }
}

#[derive(Resource)]
pub struct SpacetimeReceiver {
    pub rx: Receiver<
}
