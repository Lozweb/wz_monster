use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet2::prelude::RenetClientPlugin;
use client::entities::player_input::{client_send_input, update_mouse_coords};
use client::network::system::{client_sync_players, update_player_inputs_from_server};
use client::network::{add_netcode_network, ClientLobby, Connected, NetworkMapping};
use game_core::entities::decor::system::setup_ground;
use game_core::entities::player::component::{AimDirection, MainCamera, MouseWorldCoords, PlayerInput};
use game_core::entities::player::system::weapon_rotation_with_mouse;
use game_core::entities::player::texture::player_textures_system;
use game_core::entities::weapons::texture::weapon_texture_system;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Client".to_string(),
                resolution: (1200.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            file_path: "../assets".into(),
            ..default()
        })
    );

    app.add_plugins(RenetClientPlugin);
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());


    app.insert_resource(ClientLobby::default());
    app.insert_resource(NetworkMapping::default());
    app.insert_resource(PlayerInput::default());
    app.insert_resource(MouseWorldCoords::default());
    app.insert_resource(AimDirection::default());

    add_netcode_network(&mut app);

    app.add_systems(Update, (
        client_send_input,
        client_sync_players,
        update_player_inputs_from_server,
        update_mouse_coords,
        weapon_rotation_with_mouse
    ).in_set(Connected));

    app.add_systems(Startup, (
        setup_camera,
        setup_ground,
        player_textures_system,
        weapon_texture_system
    ));
    app.run();
}


fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}