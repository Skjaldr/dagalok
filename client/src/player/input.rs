use bevy::prelude::*;
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

    // if the player pressed a button WASD, the character moves in the direction the player indicates and also faces that direction.
    // the character can also pan the camera around with the mouse, changing the forward heading.  If the player holds the forward button "W"
    // while panning the camera, the character's direction changesn to indicate the new forward camera position is forward.
    if input.pressed(KeyCode::KeyW) {
        position.0 += *cam.forward()
    }
    if input.pressed(KeyCode::KeyS) {
        position.0 += *cam.back()
    }
    if input.pressed(KeyCode::KeyA) {
        position.0 += *cam.left()
    }
    if input.pressed(KeyCode::KeyD) {
        position.0 += *cam.right()
    }

    // set the position of y to 0.0 to lock the player's Y axis.
    position.0.y = 0.0;

    // create a delta that takes the normalized position and multiplies it by the current time and the speed of the player.
    let delta = position.0.normalize_or_zero() * time.delta_secs() * speed.0;
    transform.translation.x += delta.x;
    transform.translation.z += delta.z;

    // We want our character's position to stay the same as the direction they were last facing.  We also want to tell the IsMoving component that
    // the character is moving, if they're moving.
    if position.0.length_squared() > 0.0 {
        transform.look_to(position.0, Vec3::Y);
        moving.0 = true;
    }
}
