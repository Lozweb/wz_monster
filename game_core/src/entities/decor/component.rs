use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Ground {
    pub size: Vec2,
    pub position: Vec2,
}

impl Ground {
    pub fn new(size: Vec2, position: Vec2) -> Self {
        Self { size, position }
    }
}