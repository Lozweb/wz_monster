use bevy::prelude::{Camera2d, Commands};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
