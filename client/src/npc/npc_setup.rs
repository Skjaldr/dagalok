use bevy::prelude::*;

use crate::characters::setup_char::Health;

#[derive(Component)]
pub struct Animated;

#[derive(Component)]
pub struct NonPlayerCharacter;

#[derive(Component)]
pub struct Targettable;

#[derive(Component, Debug)]
pub struct Target(pub Option<Entity>);

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct IsMoving(pub bool);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Bundle)]
pub struct NonPlayerCharacterBundle {
    animated: Animated,
    health: Health,
    position: Position,
    speed: Speed,
    moving: IsMoving,
    npc: NonPlayerCharacter,

}

// All magic numbers are for testing only.  Will eventually move to controlled variables.
impl NonPlayerCharacterBundle {
    pub fn new(pos: Vec3, health: Health, speed: Speed, moving: bool) -> Self {
        Self {
            animated: Animated,
            health: health,
            position: Position(pos),
            speed: speed,
            moving: IsMoving(moving),
            npc: NonPlayerCharacter,
        }
    }
}
