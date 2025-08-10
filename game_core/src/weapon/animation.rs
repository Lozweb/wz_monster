use crate::player::component::{Player, PlayerChildren, PlayerInput};
use crate::texture::math::is_face_right;
use crate::weapon::component::{PivotDisk, Weapon};
use bevy::math::Quat;
use bevy::prelude::{Query, Sprite, Transform, With};

pub fn animate_weapons(
    player_query: Query<(&PlayerInput, &PlayerChildren), With<Player>>,
    mut pivot_query: Query<&mut Transform, With<PivotDisk>>,
    mut weapon_query: Query<&mut Sprite, With<Weapon>>,
) {
    for (input, player_children) in player_query.iter() {
        if let Ok(mut pivot_transform) = pivot_query.get_mut(player_children.pivot) {
            if let Ok(mut weapon_sprite) = weapon_query.get_mut(player_children.weapon) {
                move_weapon(
                    &mut pivot_transform,
                    &mut weapon_sprite,
                    input.aim_direction,
                );
            }
        }
    }
}

pub fn move_weapon(
    pivot_transform: &mut Transform,
    weapon_sprite: &mut Sprite,
    aim_direction: f32,
) {
    weapon_rotation(pivot_transform, aim_direction);
    weapon_sprite_flip(weapon_sprite, aim_direction);
}

pub fn weapon_rotation(
    transform: &mut Transform,
    aim_direction: f32,
) {
    transform.rotation = Quat::from_rotation_z(aim_direction);
    if is_face_right(aim_direction) {
        transform.translation.x = -2.5;
    } else {
        transform.translation.x = 2.5;
    }
}

pub fn weapon_sprite_flip(
    weapon_sprite: &mut Sprite,
    aim_direction: f32,
) {
    let face_right = is_face_right(aim_direction);
    if face_right {
        weapon_sprite.flip_y = !face_right;
        weapon_sprite.flip_x = face_right;
    } else {
        weapon_sprite.flip_y = !face_right;
        weapon_sprite.flip_x = !face_right;
    }
}
