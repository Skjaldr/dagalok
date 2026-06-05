use std::f32::consts::PI;
use bevy::{light::light_consts::lux, prelude::*};

pub fn setup_lights(
    mut commands: Commands,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: lux::OVERCAST_DAY,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 1.0, -PI / 4.0)),
    ));
}
