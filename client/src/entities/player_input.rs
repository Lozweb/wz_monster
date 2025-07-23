use bevy::prelude::*;
use bevy_renet2::prelude::RenetClient;
use game_core::entities::player::component::{ControlledPlayer, MainCamera, MouseWorldCoords, PlayerInput};
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

pub fn update_mouse_coords(
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Single<&Window>,
    mut mouse_world_coords: ResMut<MouseWorldCoords>,
    player: Query<&GlobalTransform, With<ControlledPlayer>>,
    mut player_input: ResMut<PlayerInput>,
    mut client: ResMut<RenetClient>,
) {
    mouse_world_coords.0 = window.cursor_position().map(|pos| {
        let (camera, camera_transform) = camera.into_inner();
        camera
            .viewport_to_world_2d(camera_transform, pos)
            .unwrap_or(vec2(0.0, 0.0))
    });

    let player_transform = player.single().ok();
    let player_pos = player_transform
        .map(|transform| transform.translation().truncate())
        .unwrap_or_default();

    let dir = mouse_world_coords.0.unwrap_or_default() - player_pos;
    if dir != Vec2::ZERO {
        player_input.aim_direction = dir.y.atan2(dir.x);
        let input_message = bincode::serialize(&*player_input).unwrap();
        client.send_message(ClientChannel::Input, input_message);
    }
}