use bevy::prelude::{Bundle, Component, Deref, Resource, Timer, Vec2};
use bevy_rapier2d::prelude::{ActiveEvents, Sensor};
use bevy_renet2::prelude::ClientId;
use serde::{Deserialize, Serialize};

pub const PLAYER_SPRITE: &str = "textures/player1.png";

#[derive(Component)]
pub struct ControlledPlayer;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub aim_direction: f32,
}
#[derive(Resource, Default)]
pub struct AimDirection(pub f32);
#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(pub Option<Vec2>);

#[derive(Debug, Component)]
pub struct Player(pub f32);

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