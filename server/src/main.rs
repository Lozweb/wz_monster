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
use game_core::entities::player::texture::player_textures_system;
use game_core::entities::weapons::texture::weapon_texture_system;
use renet2_visualizer::RenetServerVisualizer;
use server::system::decor_system::setup_camera;
use server::system::network_system::{add_netcode_network, server_network_sync, server_update_system, update_player_inputs_from_clients, ServerLobby};
use server::system::player_system::{animate_sprite, move_player_system, update_grounded_system};

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
        server_update_system,
        animate_sprite,
    ));

    app.add_systems(FixedUpdate, (
        update_grounded_system,
        move_player_system,
        server_network_sync,
        update_player_inputs_from_clients,
    ));


    app.add_systems(Startup, (
        setup_camera,
        setup_ground,
        player_textures_system,
        weapon_texture_system
    ));

    app.run();
}