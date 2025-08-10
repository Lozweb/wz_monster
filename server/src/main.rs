use bevy::app::PluginGroup;
use bevy::prelude::{default, App, AssetPlugin, ImagePlugin, Window, WindowPlugin};
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;
use bevy_renet2::prelude::RenetServerPlugin;
use server::plugin::ServerPlugin;

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
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins(ServerPlugin);

    app.run();
}