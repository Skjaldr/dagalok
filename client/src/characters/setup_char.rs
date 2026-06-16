use bevy::prelude::*;

#[derive(Component)]
pub struct Animated;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32
}

#[derive(Component, Debug)]
pub struct Target(pub Option<Entity>);

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct IsMoving(pub bool);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Bundle)]
pub struct CharacterBundle {
    animated: Animated,
    health: Health,
    position: Position,
    speed: Speed,
    moving: IsMoving,
    target: Target,

}

// All magic numbers are for testing only.  Will eventually move to controlled variables.
impl CharacterBundle {
    pub fn new(pos: Vec3, health: Health, speed: Speed, moving: bool) -> Self {
        Self {
            animated: Animated,
            health: health,
            position: Position(pos),
            speed: speed,
            moving: IsMoving(moving),
            target: Target(None), //instantiate to none
        }
    }
}
