use avian3d::{collision::collider::{Collider, ColliderConstructor, ColliderConstructorHierarchy, collider_hierarchy::RigidBodyColliders, collider_transform::ColliderTransform}, debug_render::DebugRender, dynamics::rigid_body::RigidBody, spatial_query::SpatialQueryFilter};
use bevy::{ecs::relationship::Relationship, prelude::*};
use bevy_third_person_camera::ThirdPersonCameraTarget;

use crate::{characters::setup_char::{CharacterBundle, Health, Speed}, gamestate::GameState, player::{setup_player::PlayerName, ui_hud::{get_targeted_player}}};
use crate::player::setup_player::Player;


pub fn spawn_player_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // game_assets: Res<DkGameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // if let Some(character_mesh) = gltf_handle.get(&game_assets.character_mesh) {
    let character = GltfAssetLabel::Scene(0).from_asset("models/target.glb");
    let mesh: Handle<Mesh> = asset_server.load(GltfAssetLabel::Mesh(0).from_asset("models/target.glb"));
    let materials: Handle<StandardMaterial> = asset_server.load(GltfAssetLabel::Material { index: 0, is_scale_inverted: false }.from_asset("models/target.glb"));

    let scene_handle = asset_server.load(character);
        commands.spawn((
            SceneRoot(scene_handle.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ThirdPersonCameraTarget,
            CharacterBundle::new(Vec3::new(0.0, 0.0, 0.0), Health { max: 100.0, current: 100.0 }, Speed(1.0), false),

            // Adding a Player Component marker until defining difference between NPCs and player Characaters
            Player,
            PlayerName("Joe".to_string()),
            // RigidBody::Kinematic,
            // ColliderConstructor::Capsule { radius: 0.12/2.0, height: 0.6 }
            Pickable::default(),
            DebugRender::default().with_collider_color(Color::Srgba(Srgba::RED)),



            // THE IDEA HERE IS TO ADD AN OBSERVER THAT WILL CYCLE THROUGH ALL OF THE CHILDREN OF THE ENTITY AND PICK OUT
            // THE BITS THAT WILL BE USED LATER ON SUCH AS ANIMATIONS AND MESH.  This will allow for the assets to be loaded as needed
            // instead of all at once on startup.
        ))
        .with_children(|parent| {
            parent.spawn((
                RigidBody::Kinematic,
                Collider::cuboid(0.12, 0.465, 0.12),
                Transform::from_xyz(0.0, 0.235, 0.0),
                Pickable::default(),
            ));
        }).observe(get_targeted_player::<Pointer<Click>>());

        next_state.set(GameState::LoadingCharacterAnimations);
}
