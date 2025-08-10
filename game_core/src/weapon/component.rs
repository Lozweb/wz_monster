use crate::weapon::fx_texture::PISTOL_FX_SIZE;
use bevy::math::Vec2;
use bevy::prelude::{Component, Name};
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, Friction};

#[derive(Component, Debug, Clone, Default)]
pub struct Weapon;

#[derive(Component, Debug, Clone, Default)]
pub struct PivotDisk;


const WEAPON_FX_SPEED: f32 = 1000.0;

pub fn spawn_weapon_fx_physics_bundle(
    aim_direction: f32,
) -> (
    Name,
    RigidBody,
    LockedAxes,
    Velocity,
    Collider,
    GravityScale,
    Friction,
) {
    (
        Name::new("WeaponFX Physics"),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::linear(Vec2::new(aim_direction.cos() * WEAPON_FX_SPEED, aim_direction.sin() * WEAPON_FX_SPEED)),
        Collider::ball((PISTOL_FX_SIZE.y / 2) as f32),
        GravityScale(0.0),
        Friction::coefficient(0.0),
    )
}


