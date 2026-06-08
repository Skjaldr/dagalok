use bevy::{math::VectorSpace, prelude::*};
use bevy_third_person_camera::ThirdPersonCamera;

use crate::{characters::setup_char::{Position, Speed, IsMoving}, player::setup_player::Player};

// Player controls the movement of the player character.  In this case, the player press W, A, S, or D, and the character moves in the corresponding direction
// The camera also needs to move with the character.  What is needed? Well obviously we need player input so to take in player input via Keyboard.  A direction,
// transform, and position is needed, as well as speed.  All of these are needed with a reference to time.
pub fn character_movement(
    // mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut character_query: Query<(&mut Transform, &Speed, &mut Position, &mut IsMoving), With<Player>>,
    camera_query: Query<&mut Transform, (With<ThirdPersonCamera>, Without<Player>)>,
    time: Res<Time>,

) {

    let Ok((mut transform, speed, mut position, mut moving)) = character_query.single_mut() else {
        return;
    };
    let Ok(cam) = camera_query.single() else {
        return;
    };
    // let mut direction = Vec3::ZERO;

    position.0 = Vec3::ZERO;
    moving.0 = false;

    if input.pressed(KeyCode::KeyW) {
        // moving.0 = true;
        position.0 += *cam.forward()
    }
    if input.pressed(KeyCode::KeyS) {
        // moving.0 = true;
        position.0 += *cam.back()
    }
    if input.pressed(KeyCode::KeyA) {
        // moving.0 = true;
        position.0 += *cam.left()
    }
    if input.pressed(KeyCode::KeyD) {
        // moving.0 = true;
        position.0 += *cam.right()
    }

    position.0.y = 0.0;

    let delta = position.0.normalize_or_zero() * time.delta_secs() * speed.0;
    transform.translation.x += delta.x;
    transform.translation.z += delta.z;
    if position.0.length_squared() > 0.0 {
        transform.look_to(position.0, Vec3::Y);
        moving.0 = true;
    }
}
