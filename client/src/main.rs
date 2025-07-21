use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use bevy_renet2::prelude::{RenetClient, RenetClientPlugin};
use client::entities::player_input::client_send_input;
use client::network::system::{client_sync_players, update_player_inputs_from_server};
use client::network::{add_netcode_network, ClientLobby, Connected, NetworkMapping};
use game_core::entities::decor::system::setup_ground;
use game_core::entities::player::component::PlayerInput;
use game_core::entities::player::texture::player_textures_system;
use renet2_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};

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
    app.add_plugins(bevy_egui::EguiPlugin {
        enable_multipass_for_primary_context: false,
    });
    //app.add_plugins(WorldInspectorPlugin::new());


    app.insert_resource(ClientLobby::default());
    app.insert_resource(NetworkMapping::default());
    app.insert_resource(PlayerInput::default());

    add_netcode_network(&mut app);

    app.add_systems(Update, update_visualizer_system);

    app.add_systems(Update, (
        client_send_input,
        client_sync_players,
        update_player_inputs_from_server
    ).in_set(Connected));

    app.insert_resource(RenetClientVisualizer::<200>::new(RenetVisualizerStyle::default()));

    app.add_systems(Startup, (
        setup_camera,
        setup_ground,
        player_textures_system
    ));
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn update_visualizer_system(
    mut egui_contexts: EguiContexts,
    mut visualizer: ResMut<RenetClientVisualizer<200>>,
    client: Res<RenetClient>,
    mut show_visualizer: Local<bool>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    visualizer.add_network_info(client.network_info());
    if keyboard_input.just_pressed(KeyCode::F1) {
        *show_visualizer = !*show_visualizer;
    }
    if *show_visualizer {
        visualizer.show_window(egui_contexts.ctx_mut());
    }
}