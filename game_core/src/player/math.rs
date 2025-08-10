use crate::player::component::{Grounded, JumpCounter, Player, PlayerInput};
use bevy::math::Vec2;
use bevy_rapier2d::dynamics::Velocity;

pub fn apply_velocity(
    player: &Player,
    input: &PlayerInput,
    velocity: &mut Velocity,
) {
    let x_axis = -(input.left as i8) + input.right as i8;
    let mut move_delta = Vec2::new(x_axis as f32, 0.0);
    if move_delta != Vec2::ZERO {
        move_delta = move_delta.normalize();
    }

    velocity.linvel.x = move_delta.x * player.speed;
}

pub fn apply_jump_velocity(
    player: &Player,
    input: &PlayerInput,
    velocity: &mut Velocity,
    jump_counter: &mut JumpCounter,
    grounded: &Grounded,
) {
    if input.jump && jump_counter.use_jump() {
        velocity.linvel.y = player.speed * 1.5;
    } else if !grounded.0 {
        velocity.linvel.y -= player.speed * 0.1;
    }
}