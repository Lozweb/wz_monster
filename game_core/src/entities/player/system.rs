use crate::entities::player::component::{ControlledPlayer, PlayerInput};
use bevy::math::Quat;
use bevy::prelude::{Children, GlobalTransform, Name, Query, Res, Transform, Vec3, With};

pub fn weapon_rotation_with_mouse(
    player_query: Query<(&GlobalTransform, &Children), With<ControlledPlayer>>,
    mut disk_query: Query<&mut Transform, With<Name>>,
    player_input: Res<PlayerInput>,
) {
    let Ok((_, children)) = player_query.single() else { return };

    for &child in children {
        if let Ok(mut transform) = disk_query.get_mut(child) {
            transform.rotation = Quat::from_rotation_z(player_input.aim_direction);
        }
    }
}

pub fn rand_spawn_player_position() -> Vec3 {
    Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0)
}