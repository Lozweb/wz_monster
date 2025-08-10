use bevy::prelude::*;
use bevy_renet2::prelude::RenetClient;
use game_core::network::network::ClientChannel;
use game_core::player::component::{ControlledPlayer, MouseWorldCoords, PlayerInput};

const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
const JUMP: KeyCode = KeyCode::Space;
const SHOOT: MouseButton = MouseButton::Left;


pub fn send_input(
    mut player_input: ResMut<PlayerInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut client: ResMut<RenetClient>,
) {
    player_input.up = keyboard_input.any_pressed(UP);
    player_input.down = keyboard_input.any_pressed(DOWN);
    player_input.left = keyboard_input.any_pressed(LEFT);
    player_input.right = keyboard_input.any_pressed(RIGHT);
    player_input.jump = keyboard_input.just_pressed(JUMP);
    player_input.shoot = mouse_input.just_pressed(SHOOT);

    let input_message = match bincode::serialize(&*player_input) {
        Ok(msg) => msg,
        Err(e) => {
            error!("Erreur de sérialisation de PlayerInput: {:?}", e);
            return;
        }
    };
    client.send_message(ClientChannel::Input, input_message);
}


#[derive(Component)]
pub struct MainCamera;

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

        match bincode::serialize(&*player_input) {
            Ok(input_message) => client.send_message(ClientChannel::Input, input_message),
            Err(e) => error!("Erreur de sérialisation de PlayerInput: {:?}", e),
        }
    }
}