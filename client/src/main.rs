use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use game_core::entities::decor::system::setup_ground;
use wz_monster::entities::player::PlayerPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "WZ Monster".to_string(),
            resolution: (1200.0, 720.0).into(),
            ..default()
        }),
        ..default()
    }).set(ImagePlugin::default_nearest()));
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_plugins(PlayerPlugin);

    app.add_systems(Startup, (setup_camera, setup_ground));
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}


