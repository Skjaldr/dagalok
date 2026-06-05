use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCamera;


pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Transform::from_xyz(2.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        ThirdPersonCamera::default(),
    ));
}
