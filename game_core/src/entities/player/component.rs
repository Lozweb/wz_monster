use bevy::prelude::{Bundle, Component, Handle, Image, Resource, Timer};
use bevy_rapier2d::prelude::{ActiveEvents, Sensor};
use bevy_renet2::prelude::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

#[derive(Debug, Component)]
pub struct Player(pub f32);

#[derive(Resource, Clone)]
pub struct PlayerTexture(pub Handle<Image>);

#[derive(Component, Default)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct JumpCounter {
    pub jumps_left: u8,
    pub max_jumps: u8,
}
impl JumpCounter {
    pub fn reset(&mut self) {
        self.jumps_left = self.max_jumps;
    }

    pub fn use_jump(&mut self) -> bool {
        if self.jumps_left > 0 {
            self.jumps_left -= 1;
            true
        } else {
            false
        }
    }
}
#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Bundle)]
pub struct SensorBundle {
    pub sensor: Sensor,
    pub active_events: ActiveEvents,

}
impl Default for SensorBundle {
    fn default() -> Self {
        Self {
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Debug, Component)]
pub struct PlayerNetwork {
    pub id: ClientId,
}
