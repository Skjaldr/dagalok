use petname::Generator;
use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table};

const SPAWN_X: f32 = -5.0;
const SPAWN_Y: f32 = 0.0;
const SPAWN_Z: f32 = -5.0;

#[spacetimedb::table(accessor = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    #[unique]
    username: String,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    is_online: bool,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum CharacterType {
    Player,
    Npc,
}

#[derive(SpacetimeType, Clone, Copy, Debug)]
pub struct DbVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(SpacetimeType, Clone, Copy, Debug)]
pub struct CharacterHealth {
    pub max: f32,
    pub current: f32,
}

#[spacetimedb::table( accessor = character, public)]
pub struct CharacterTable {
    #[primary_key]
    pub npc_id: i64,
    pub name: String,
    pub position: DbVec3,
    pub health: CharacterHealth,
    pub speed: f32,
    pub is_moving: bool,
    pub is_targettable: bool,
    pub character_type: CharacterType,
}

#[spacetimedb::reducer]
pub fn spawn_npc(ctx: &ReducerContext, pos: DbVec3) {
    let unique_id = ctx.timestamp.to_micros_since_unix_epoch();


    ctx.db.character().insert(CharacterTable {
        npc_id: unique_id,
        name: generate_username(ctx),
        position: pos,
        speed: 1.0,
        is_targettable: true,
        health: CharacterHealth {
            current: 100.0,
            max: 100.0,
        },
        is_moving: false,
        character_type: CharacterType::Npc,
    });
 }

fn generate_username(ctx: &ReducerContext) -> String {
    let mut rng = ctx.rng();
    let petnames = petname::Petnames::default();
    petnames
        .generate(&mut rng, 3, "-")
        .unwrap_or_else(|| format!("player-{}", ctx.timestamp.to_micros_since_unix_epoch()))
}

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    log::info!("Server module initialized");
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    let sender = ctx.sender();
    if let Some(player) = ctx.db.player().identity().find(sender) {
        log::info!("Player '{}' reconnected", player.username);
        ctx.db.player().identity().update(Player {
            is_online: true,
            ..player
        });
    } else {
        let username = generate_username(ctx);
        log::info!("New Player '{}' connected", username);
        ctx.db.player().insert(Player {
            identity: sender,
            username,
            position_x: SPAWN_X,
            position_y: SPAWN_Y,
            position_z: SPAWN_Z,
            is_online: true,
        });
    }
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    let sender = ctx.sender();
    if let Some(player) = ctx.db.player().identity().find(sender) {
        log::info!("Player {} disconnected", player.username);
        ctx.db.player().identity().update(Player {
            is_online: false,
            ..player
        });
    }
}

#[spacetimedb::reducer]
pub fn register_player(
        ctx: &ReducerContext,
        username: String
) -> Result<(), String> {
    if username.is_empty() {
        return Err("Username must not be empty".to_string());
    }
    if username.len() > 32 {
        return Err("Username cannot be longer than 32 characters".to_string());
    }

    let sender = ctx.sender();

    if ctx.db.player().username().find(&username).is_some_and(|p| p.identity != sender) {
        return Err(format!("{} is already taken", username));
    }

    if let Some(player) = ctx.db.player().identity().find(sender) {
        log::info!("Player {} renamed to {}", player.username, username);
        ctx.db.player().identity().update(Player {
            username,
            ..player
        });
        Ok(())
    } else {
        Err("Cannot rename: player not found. Connect first".to_string())
    }
}
