use bevy::asset::Assets;
use bevy::image::TextureAtlasLayout;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{ChildOf, Children, Commands, ContainsEntity, EventReader, GlobalTransform, Quat, Query, Res, ResMut, Sprite, Time, Transform, With};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::pipeline::CollisionEvent;
use game_core::entities::player::component::{spawn_weapon_fx, AnimationIndices, AnimationTimer, Grounded, JumpCounter, Player, PlayerInput, PlayerNetwork, PlayerWeaponSelected};
use game_core::entities::player::system::{is_face_right, radian_to_degrees, weapon_sprite_flip};
use game_core::entities::player::weapon_fx_texture::{WeaponFxTextureEntityType, WeaponFxTextures};
use game_core::entities::player::weapon_texture::{PivotDisk, Weapon};

pub fn player_move(
    mut query: Query<(
        &Player,
        &PlayerInput,
        &mut Velocity,
        &Grounded,
        &mut JumpCounter
    )>
) {
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

pub fn player_jump_control(
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

pub fn animate_players(
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
                        (index, _) if index >= indices.last as usize => indices.first as usize,
                        (index, true) => index + 1,
                        (0, false) => indices.last as usize,
                        (index, false) => index - 1,
                    };
                }
            }
        } else if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = indices.first as usize;
        }

        sprite.flip_x = face_right;
    }
}

pub fn animate_weapons(
    player_query: Query<(&PlayerInput, &Children), With<Player>>,
    mut pivot_query: Query<(&Children, &mut Transform), With<PivotDisk>>,
    mut weapon_query: Query<&mut Sprite, With<Weapon>>,
) {
    for (input, player_children) in player_query.iter() {
        for &pivot_entity in player_children.iter() {
            if let Ok(mut weapon) = pivot_query.get_mut(pivot_entity) {
                weapon.1.rotation = Quat::from_rotation_z(input.aim_direction);
                if is_face_right(radian_to_degrees(input.aim_direction)) {
                    weapon.1.translation.x = -2.5;
                } else {
                    weapon.1.translation.x = 2.5;
                }
                for &weapon_entity in weapon.0.iter() {
                    if let Ok(mut weapon_sprite) = weapon_query.get_mut(weapon_entity) {
                        weapon_sprite_flip(&mut weapon_sprite, input.aim_direction);
                    }
                }
            }
        }
    }
}
pub fn player_shoot(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut weapon_fx_textures: Res<WeaponFxTextures>,
    player_query: Query<(&PlayerInput, &PlayerWeaponSelected, &Children, &PlayerNetwork), With<Player>>,
    pivot_disk_query: Query<&Children, With<PivotDisk>>,
    weapon_query: Query<&GlobalTransform, With<Weapon>>,
) {
    for (player_input, player_weapon_selected, children, player_network) in player_query.iter() {
        if player_input.shoot {
            for children in children.iter() {
                if let Ok(pivot_children) = pivot_disk_query.get(children.entity()) {
                    if let Some(weapon_entity) = pivot_children.first() {
                        if let Ok(global_transform) = weapon_query.get(*weapon_entity) {
                            let offset = Vec3::new(32.5, 0.0, 0.0);
                            let position = global_transform.translation() + if is_face_right(radian_to_degrees(player_input.aim_direction)) {
                                offset
                            } else {
                                -offset
                            };
                            spawn_weapon_fx(
                                &mut commands,
                                &mut texture_atlas_layouts,
                                &mut weapon_fx_textures,
                                position,
                                &WeaponFxTextureEntityType::from(&player_weapon_selected.weapon_entity_type),
                                player_input.aim_direction,
                                player_network.id,
                            );
                        }
                    }
                }
            }
        }
    }
}