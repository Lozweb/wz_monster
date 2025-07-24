use crate::entities::player::component::{ControlledPlayer, PlayerInput};
use crate::entities::weapons::component::{PivotDisk, Weapon};
use crate::entities::weapons::system::weapon_rotate_and_flip;
use bevy::math::Quat;
use bevy::prelude::{Children, Query, Res, Sprite, Transform, Vec3, With};


// todo : include in server_network_sync from server side
// update on update_player_inputs_from_server from client side
pub fn weapon_rotation_with_mouse(
    player_query: Query<&Children, With<ControlledPlayer>>,
    mut disk_query: Query<(&mut Transform, &Children), With<PivotDisk>>,
    mut weapon_query: Query<&mut Sprite, With<Weapon>>,
    player_input: Res<PlayerInput>,
) {
    let Ok(children) = player_query.single() else { return };

    for &child in children {
        if let Ok(mut weapon) = disk_query.get_mut(child) {
            weapon.0.rotation = Quat::from_rotation_z(player_input.aim_direction);
            for &weapon_entity in weapon.1.iter() {
                if let Ok(mut weapon_sprite) = weapon_query.get_mut(weapon_entity) {
                    weapon_rotate_and_flip(&mut weapon_sprite, player_input.aim_direction);
                }
            }
        }
    }
}


pub fn rand_spawn_player_position() -> Vec3 {
    Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0)
}