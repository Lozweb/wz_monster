use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet2::prelude::RenetClientPlugin;
use client::plugin::ClientPlugin;

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
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(ClientPlugin);
    app.run();
}


