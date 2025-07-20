use bevy::prelude::*;
use game_core::entities::player::component::{Player, PlayerInput};

pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_info: Query<(&Player, &mut PlayerInput)>,
) {
    for (_, mut input) in &mut player_info {
        input.up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
        input.down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
        input.left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        input.right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
        input.jump = keyboard_input.just_pressed(KeyCode::Space);
    }
}



