use bevy::math::Vec2;
use bevy::prelude::{ChildOf, Children, EventReader, Quat, Query, Res, Sprite, Time, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::pipeline::CollisionEvent;
use game_core::entities::player::component::{AnimationIndices, AnimationTimer, Grounded, JumpCounter, Player, PlayerInput};
use game_core::entities::weapons::component::{PivotDisk, Weapon};
use game_core::entities::weapons::system::{is_face_right, radian_to_degrees, weapon_rotate_and_flip};


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

pub fn players_animate(
    time: Res<Time>,
    player_query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite, &PlayerInput), With<Player>>,
) {
    for (indices, mut timer, mut sprite, input) in player_query {
        let player_move = input.left || input.right;
        let face_right = is_face_right(radian_to_degrees(input.aim_direction));
        if player_move {
            timer.0.tick(time.delta());
            if timer.0.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = match (atlas.index, face_right) {
                        (index, _) if index >= indices.last => indices.first,
                        (index, true) => index + 1,
                        (0, false) => indices.last,
                        (index, false) => index - 1,
                    };
                }
            }
        } else if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = indices.first;
        }

        sprite.flip_x = face_right;
    }
}

pub fn weapons_animate(
    player_query: Query<(&PlayerInput, &Children), With<Player>>,
    mut pivot_query: Query<(&Children, &mut Transform), With<PivotDisk>>,
    mut weapon_query: Query<&mut Sprite, With<Weapon>>,
) {
    for (input, player_children) in player_query.iter() {
        for &pivot_entity in player_children.iter() {
            if let Ok(mut weapon) = pivot_query.get_mut(pivot_entity) {
                weapon.1.rotation = Quat::from_rotation_z(input.aim_direction);

                for &weapon_entity in weapon.0.iter() {
                    if let Ok(mut weapon_sprite) = weapon_query.get_mut(weapon_entity) {
                        weapon_rotate_and_flip(&mut weapon_sprite, input.aim_direction);
                    }
                }
            }
        }
    }
}
