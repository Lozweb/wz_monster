use bevy::prelude::{Bundle, Component, Name, Resource, Timer, TimerMode};
use bevy_rapier2d::prelude::{ActiveEvents, LockedAxes, Sensor, Velocity};
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

#[derive(Component)]
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


#[derive(Bundle)]
pub struct ClientPlayerBundle {
    name: Name,
    locked_axes: LockedAxes,
    velocity: Velocity,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    player: Player,
    player_input: PlayerInput,
    grounded: Grounded,
    jump_counter: JumpCounter,
}

pub fn create_client_player_bundle(client_id: &ClientId) -> ClientPlayerBundle {
    ClientPlayerBundle {
        name: Name::new(format!("Player_{client_id}")),
        locked_axes: LockedAxes::ROTATION_LOCKED,
        velocity: Velocity::zero(),
        animation_indices: AnimationIndices { first: 1, last: 3 },
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        player: Player(350.),
        player_input: PlayerInput::default(),
        grounded: Grounded(false),
        jump_counter: JumpCounter { jumps_left: 2, max_jumps: 2 },
    }
}