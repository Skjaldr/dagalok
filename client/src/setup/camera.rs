use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCamera;

use crate::gamestate::GameState;


pub fn setup_camera(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Transform::from_xyz(2.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        ThirdPersonCamera::default(),
    ));
    next_state.set(GameState::LoadingLights);
}
