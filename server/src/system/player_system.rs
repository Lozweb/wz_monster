use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::{ChildOf, EventReader, Query, Res, Sprite, Time, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::pipeline::CollisionEvent;
use game_core::entities::player::component::{AnimationIndices, AnimationTimer, Grounded, JumpCounter, Player, PlayerInput};

pub fn move_player_system(mut query: Query<(&Player, &PlayerInput, &mut Velocity, &Grounded, &mut JumpCounter)>) {
    for (player, input, mut velocity, grounded, mut jump_counter) in query.iter_mut() {
        let x_axis = -(input.left as i8) + input.right as i8;
        let mut move_delta = Vec2::new(x_axis as f32, 0.0);
        if move_delta != Vec2::ZERO {
            move_delta = move_delta.normalize();
        }

        velocity.linvel.x = move_delta.x * player.0;

        if input.jump && jump_counter.use_jump() {
            velocity.linvel.y = player.0 * 2.5;
        } else if !grounded.0 {
            velocity.linvel.y -= player.0 * 0.1;
        }
    }
}

pub fn update_grounded_system(
    mut collision_events: EventReader<CollisionEvent>,
    child_of: Query<&ChildOf>,
    mut grounded_query: Query<(&mut Grounded, &mut JumpCounter)>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) | CollisionEvent::Stopped(e1, e2, _) => {
                let is_grounded = matches!(event, CollisionEvent::Started(_, _, _));
                for entity in [e1, e2] {
                    if let Ok(child) = child_of.get(*entity) {
                        if let Ok((mut grounded, mut jump_counter)) = grounded_query.get_mut(child.parent()) {
                            grounded.0 = is_grounded;
                            if is_grounded { jump_counter.reset(); }
                        }
                    }
                }
            }
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite, &PlayerInput), With<Player>>,
) {
    for (indices, mut timer, mut sprite, input) in &mut query {
        let player_move = input.left || input.right;
        if player_move {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index >= indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
        } else if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = indices.first;
        }

        if input.left {
            sprite.flip_x = false;
        } else if input.right {
            sprite.flip_x = true;
        }

        info!("aim_direction: {}", input.aim_direction);
    }
}

