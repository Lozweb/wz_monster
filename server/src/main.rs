use bevy::app::{FixedUpdate, PluginGroup, Startup};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{default, App, AssetPlugin, Events, ImagePlugin, Update, Window, WindowPlugin};
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::{CollisionEvent, RapierDebugRenderPlugin};
use bevy_renet2::prelude::RenetServerPlugin;
use game_core::entities::decor::system::setup_ground;
use game_core::entities::player::player_texture::load_player_textures;
use game_core::entities::player::weapon_texture::load_weapon_textures;
use renet2_visualizer::RenetServerVisualizer;
use server::system::decor_system::setup_camera;
use server::system::network_system::{add_netcode_network, server_event, server_network_sync, update_player_inputs_from_clients, ServerLobby};
use server::system::player_system::{animate_players, animate_weapons, player_jump_control, player_move};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Server".to_string(),
                resolution: (1200.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            file_path: "../assets".into(),
            ..default()
        }));

    app.add_plugins(RenetServerPlugin);
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.insert_resource(ServerLobby::default());
    app.insert_resource(RenetServerVisualizer::<200>::default());
    app.init_resource::<Events<CollisionEvent>>();

    add_netcode_network(&mut app);


    app.add_systems(Update, (
        server_event,
        animate_players,
        animate_weapons,
        player_jump_control,
        player_move,
    ));

    app.add_systems(FixedUpdate, (
        server_network_sync,
        update_player_inputs_from_clients,
    ));


    app.add_systems(Startup, (
        load_player_textures,
        load_weapon_textures,
        setup_camera,
        setup_ground,
    ));

    app.run();
}