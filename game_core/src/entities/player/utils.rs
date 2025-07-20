use bevy::prelude::Vec3;

pub fn rand_player_position() -> Vec3 {
    Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0)
}