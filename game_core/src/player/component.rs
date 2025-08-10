use crate::weapon::texture::WeaponTextureType;
use bevy::prelude::{Bundle, Commands, Component, Deref, Entity, Name, Resource, Timer, Transform, Vec2};
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, Friction};
use bevy_rapier2d::prelude::{ActiveEvents, Sensor};
use bevy_renet2::prelude::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component)]
pub struct PlayerNetwork {
    pub id: ClientId,
}

#[derive(Debug, Resource)]
pub struct CurrentClientId(pub u64);

#[derive(Component)]
pub struct ControlledPlayer;

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct PlayerChildren {
    pub pivot: Entity,
    pub weapon: Entity,
    pub sensor: Option<Entity>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub aim_direction: f32,
    pub shoot: bool,
}

#[derive(Resource, Default)]
pub struct AimDirection(pub f32);

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(pub Option<Vec2>);


#[derive(Component)]
pub struct JumpCounter {
    pub jumps_left: u8,
    pub max_jumps: u8,
}
impl JumpCounter {
    pub fn reset(&mut self) {
        self.jumps_left = self.max_jumps;
    }

    pub fn use_jump(&mut self) -> bool {
        if self.jumps_left > 0 {
            self.jumps_left -= 1;
            true
        } else {
            false
        }
    }
}
#[derive(Component)]
pub struct AnimationIndices {
    pub first: u32,
    pub last: u32,
}
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Default)]
pub struct Grounded(pub bool);

#[derive(Bundle)]
pub struct SensorBundle {
    pub sensor: Sensor,
    pub active_events: ActiveEvents,

}
impl Default for SensorBundle {
    fn default() -> Self {
        Self {
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerWeaponSelected {
    pub weapon_entity_type: WeaponTextureType,
}

impl PlayerWeaponSelected {
    pub fn default_weapon() -> Self {
        Self {
            weapon_entity_type: WeaponTextureType::Pistol,
        }
    }
}

pub fn player_physics() -> (
    RigidBody,
    LockedAxes,
    Velocity,
    Collider,
    GravityScale,
    Friction,
) {
    let points = vec![
        Vec2::new(0.0, 80.0),     // haut
        Vec2::new(30.0, 40.0),    // haut droite
        Vec2::new(59.0, 0.0),     // droite
        Vec2::new(30.0, -40.0),   // bas droite
        Vec2::new(0.0, -80.0),    // bas
        Vec2::new(-30.0, -40.0),  // bas gauche
        Vec2::new(-59.0, 0.0),    // gauche
        Vec2::new(-30.0, 40.0),   // haut gauche
    ];

    (
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        Collider::convex_hull(&points).unwrap(),
        GravityScale(2.5),
        Friction::coefficient(0.0),
    )
}


pub fn spawn_player_sensor(
    commands: &mut Commands,
) -> Entity {
    commands.spawn((
        Name::new("Player Sensor"),
        Sensor,
        Collider::cuboid(10.0, 2.0),
        ActiveEvents::COLLISION_EVENTS,
        Transform::from_xyz(0.0, -82.0, 0.0),
    )).id()
}
