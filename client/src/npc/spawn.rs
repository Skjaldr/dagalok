use avian3d::{collision::collider::Collider, debug_render::DebugRender, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;


use crate::{characters::setup_char::{Health, Targettable}, gamestate::GameState, module_bindings::{DbVec3, spawn_npc}, networking::SpacetimeConnection, npc::npc_setup::{NonPlayerCharacterBundle, Speed}, ui::systems::get_target};

pub fn spawn_npc_physical(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let character = GltfAssetLabel::Scene(0).from_asset("models/target.glb");
    let scene_handle = asset_server.load(character);
    commands.spawn((

        SceneRoot(scene_handle.clone()),
        // Transform::from_xyz(-5.0, 0.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        NonPlayerCharacterBundle::new(DbVec3 { x: 0.0, y: 0.0, z: 0.0 }, Health { max: 100.0, current: 100.0 }, Speed(1.0), false),
        Pickable::default(),
        Targettable,
        DebugRender::default().with_collider_color(Color::Srgba(Srgba::RED)),
    ))
    .with_children(|parent| {
        parent.spawn((
            RigidBody::Kinematic,
            Collider::cuboid(0.12, 0.465, 0.12),
            Transform::from_xyz(0.0, 0.235, 0.0),
        ));
    }).observe(get_target::<Pointer<Click>>());

}

pub fn spawn_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    connection: Res<SpacetimeConnection>
) {
    if let Err(e) = connection.conn.reducers.spawn_npc(DbVec3 {x: 0.0, y: 0.0, z: 0.0}) {
        error!("Failed to send spawn_npc reducer request: {:?}", e)
    } else {
        info!("Spawn request successfully sent to the server");
        spawn_npc_physical(commands, asset_server);
    }
}
