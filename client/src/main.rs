use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet2::prelude::RenetClientPlugin;
use client::entities::player::handle_player_input;
use client::network::system::{client_send_input, client_sync_players, update_player_inputs_from_server};
use client::network::{add_netcode_network, ClientLobby, Connected, NetworkMapping};
use game_core::entities::decor::system::setup_ground;
use game_core::entities::player::component::PlayerInput;
use game_core::entities::player::system::setup_player_texture;
use renet2_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Client".to_string(),
            resolution: (1200.0, 720.0).into(),
            ..default()
        }),
        ..default()
    }).set(ImagePlugin::default_nearest()));

    app.add_plugins(RenetClientPlugin);
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());


    app.insert_resource(ClientLobby::default());
    app.insert_resource(NetworkMapping::default());
    app.insert_resource(PlayerInput::default());

    add_netcode_network(&mut app);

    app.add_systems(Update, (
        handle_player_input,
        client_send_input,
        client_sync_players,
        update_player_inputs_from_server
    ).in_set(Connected));

    app.insert_resource(RenetClientVisualizer::<200>::new(RenetVisualizerStyle::default()));

    app.add_systems(Startup, (
        setup_camera,
        setup_ground,
        setup_player_texture
    ));
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}