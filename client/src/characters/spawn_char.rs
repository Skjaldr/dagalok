use avian3d::{collision::collider::Collider, debug_render::DebugRender, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;


use crate::{characters::setup_char::{CharacterBundle, Health, Speed, Targettable}, gamestate::GameState, player::setup_player::PlayerName, ui::{self, systems::get_target}};
use crate::player::setup_player::Player;


pub fn spawn_player_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let character = GltfAssetLabel::Scene(0).from_asset("models/target.glb");

    let scene_handle = asset_server.load(character);
        commands.spawn((
            SceneRoot(scene_handle.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ThirdPersonCameraTarget,
            CharacterBundle::new(Vec3::new(0.0, 0.0, 0.0), Health { max: 100.0, current: 100.0 }, Speed(1.0), false),

            // Adding a Player Component marker until defining difference between NPCs and player Characaters
            Player,
            PlayerName("Joe".to_string()),
            Pickable::default(),
            DebugRender::default().with_collider_color(Color::Srgba(Srgba::RED)),
            Targettable,
        ))
        .with_children(|parent| {
            parent.spawn((
                RigidBody::Kinematic,
                Collider::cuboid(0.12, 0.465, 0.12),
                Transform::from_xyz(0.0, 0.235, 0.0),
            ));
        })
        .observe(get_target::<Pointer<Click>>());

        next_state.set(GameState::LoadingCharacterAnimations);
}
