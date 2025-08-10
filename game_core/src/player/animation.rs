use crate::player::component::{AnimationIndices, AnimationTimer, Player, PlayerInput};
use crate::texture::math::is_face_right;
use bevy::prelude::{Query, Res, Sprite, With};
use bevy::time::Time;
pub fn animate_players(
    time: Res<Time>,
    player_query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite, &PlayerInput), With<Player>>,
) {
    for (indices, mut animation_timer, mut sprite, input) in player_query {
        player_sprite_animation(&time, indices, &mut animation_timer, &mut sprite, input);
    }
}
pub fn player_sprite_animation(
    time: &Res<Time>,
    indices: &AnimationIndices,
    timer: &mut AnimationTimer,
    sprite: &mut Sprite,
    input: &PlayerInput,
) {
    let player_move = input.left || input.right;
    let face_right = is_face_right(input.aim_direction);

    sprite.flip_x = face_right;

    if player_move {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = next_animation_index(atlas.index, face_right, indices);
            }
        }
    } else if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = indices.first as usize;
    }
}

fn next_animation_index(current_index: usize, face_right: bool, indices: &AnimationIndices) -> usize {
    match (current_index, face_right) {
        (index, _) if index >= indices.last as usize => indices.first as usize,
        (index, true) => index + 1,
        (0, false) => indices.last as usize,
        (index, false) => index - 1,
    }
}