use bevy::app::{FixedUpdate, PluginGroup, Startup};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{default, App, AssetPlugin, Events, ImagePlugin, Res, ResMut, Update, Window, WindowPlugin};
use bevy::DefaultPlugins;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_renet2::prelude::{RenetServer, RenetServerPlugin};
use game_core::entities::decor::system::setup_ground;
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
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: false,
    });
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    //app.add_plugins(WorldInspectorPlugin::new());
    //app.add_plugins(RapierDebugRenderPlugin::default());

    app.insert_resource(ServerLobby::default());
    app.insert_resource(RenetServerVisualizer::<200>::default());
    app.init_resource::<Events<CollisionEvent>>();

    add_netcode_network(&mut app);


    app.add_systems(Update, (
        server_update_system,
        animate_sprite,
        update_visualizer_system
    ));

    app.add_systems(FixedUpdate, (
        update_grounded_system,
        move_player_system,
        server_network_sync,
        update_player_inputs_from_clients,
    ));


    app.add_systems(Startup, (
        setup_camera,
        setup_ground
    ));

    app.run();
}

fn update_visualizer_system(mut egui_contexts: EguiContexts, mut visualizer: ResMut<RenetServerVisualizer<200>>, server: Res<RenetServer>) {
    visualizer.update(&server);
    visualizer.show_window(egui_contexts.ctx_mut());
}
