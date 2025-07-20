use bevy::prelude::*;
use game_core::entities::player::component::PlayerInput;

pub fn handle_player_input(
    mut player_input: ResMut<PlayerInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    player_input.up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    player_input.down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    player_input.left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    player_input.right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    player_input.jump = keyboard_input.just_pressed(KeyCode::Space);
}





