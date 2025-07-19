use crate::entities::player::component::*;
use crate::entities::player::player_input::PlayerInput;
use bevy::math::Vec2;
use bevy::prelude::{ChildOf, EventReader, Query, Res, Sprite, Time, With};
use bevy_rapier2d::prelude::{CollisionEvent, Velocity};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite, &Velocity), With<Player>>,
) {
    for (indices, mut timer, mut sprite, velocity) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last || velocity.linvel.x.abs() < 0.1 {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}


pub fn player_movement(
    mut player_data: Query<(&Player, &PlayerInput, &mut Velocity, &Grounded, &mut JumpCounter, &mut Sprite)>,
) {
    for (player, input, mut velocity, grounded, mut jump_counter, mut sprite) in player_data.iter_mut() {
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

pub fn update_grounded_system(
    mut collision_events: EventReader<CollisionEvent>,
    child_of: Query<&ChildOf>,
    mut grounded_query: Query<(&mut Grounded, &mut JumpCounter)>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) | CollisionEvent::Stopped(e1, e2, _) => {
                let flag = matches!(event, CollisionEvent::Started(_, _, _));
                for entity in [e1, e2] {
                    if let Ok(child) = child_of.get(*entity) {
                        if let Ok((mut grounded, mut jump_counter)) = grounded_query.get_mut(child.parent()) {
                            grounded.0 = flag;
                            if flag {
                                jump_counter.reset();
                            }
                        }
                    }
                }
            }
        }
    }
}
