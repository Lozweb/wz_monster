use bevy::prelude::*;
use game_core::entities::player::component::{create_player, Player, PlayerInput};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_player)
            .add_systems(FixedUpdate, (
                handle_player_input,
            ));

        app.insert_resource(PlayerInput::default());
    }
}

fn handle_player_input(
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



