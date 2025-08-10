use crate::weapon::fx_texture::{FxComponent, PISTOL_FX_SIZE};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Entity, Name, Query, Transform, With};
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, CollisionGroups, Friction, Group};
use bevy_renet2::prelude::ClientId;

#[derive(Component, Debug, Clone, Default)]
pub struct Weapon;

#[derive(Component, Debug, Clone, Default)]
pub struct PivotDisk;


const WEAPON_FX_SPEED: f32 = 1000.0;

pub fn spawn_weapon_fx_physics_bundle(
    aim_direction: f32,
    ignore_player_group: ClientId,
) -> (
    Name,
    RigidBody,
    LockedAxes,
    Velocity,
    Collider,
    GravityScale,
    Friction,
    CollisionGroups,
) {
    (
        Name::new("WeaponFX Physics"),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::linear(Vec2::new(aim_direction.cos() * WEAPON_FX_SPEED, aim_direction.sin() * WEAPON_FX_SPEED)),
        Collider::ball((PISTOL_FX_SIZE.y / 2) as f32),
        GravityScale(0.0),
        Friction::coefficient(0.0),
        CollisionGroups::new(
            Group::from_bits_truncate(!ignore_player_group as u32),
            Group::from_bits_truncate(u32::MAX),
        ),
    )
}


pub fn despawn_weapon_fx_out_of_screen_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<FxComponent>>,
) {
    let max_distance: f32 = 500.0;
    for (entity, transform) in query.iter() {
        let distance = transform.translation.length();
        if distance > max_distance {
            commands.entity(entity).despawn();
        }
    }
}