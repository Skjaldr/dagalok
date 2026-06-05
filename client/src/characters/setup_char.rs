use bevy::prelude::*;

#[derive(Component)]
pub struct Animated;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32
}

#[derive(Component)]
pub struct Position {
    pos: Vec3,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Bundle)]
pub struct CharacterBundle {
    animated: Animated,
    health: Health,
    position: Position,
    speed: Speed,
}

// All magic numbers are for testing only.  Will move to controlled variables.
impl CharacterBundle {
    pub fn new(pos: Vec3, health: Health, speed: Speed) -> Self {
        Self {
            animated: Animated,
            health: health,
            position: Position { pos: pos },
            speed: speed,
        }
    }
}
