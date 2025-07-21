use bevy::prelude::*;
use bevy_renet2::prelude::RenetClient;
use game_core::entities::player::component::PlayerInput;
use game_core::network::network_entities::ClientChannel;

pub fn client_send_input(
    mut player_input: ResMut<PlayerInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut client: ResMut<RenetClient>,
) {
    player_input.up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    player_input.down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    player_input.left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    player_input.right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    player_input.jump = keyboard_input.just_pressed(KeyCode::Space);

    if player_input.is_changed() {
        let input_message = bincode::serialize(&*player_input).unwrap();
        client.send_message(ClientChannel::Input, input_message);
    }
}