use bevy::math::Vec2;
use bevy::prelude::{Query, Sprite};
use bevy_rapier2d::dynamics::Velocity;
use game_core::entities::player::component::{Grounded, JumpCounter, Player, PlayerInput};

pub fn move_player_system(mut query: Query<(&Player, &PlayerInput, &mut Velocity, &Grounded, &mut JumpCounter, &mut Sprite)>) {
    for (player, input, mut velocity, grounded, mut jump_counter, mut sprite) in query.iter_mut() {
        let x_axis = -(input.left as i8) + input.right as i8;
        let mut move_delta = Vec2::new(x_axis as f32, 0.0);
        if move_delta != Vec2::ZERO {
            move_delta = move_delta.normalize();
        }

        if input.left {
            sprite.flip_x = false;
        } else if input.right {
            sprite.flip_x = true;
        }

        velocity.linvel.x = move_delta.x * player.0;

        if input.jump && jump_counter.use_jump() {
            velocity.linvel.y = player.0 * 2.5;
        } else if !grounded.0 {
            velocity.linvel.y -= player.0 * 0.1;
        }
    }
}