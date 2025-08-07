use crate::entities::player::player_texture::{player_texture_entity_to_handle, PlayerTextureEntity, PlayerTextureEntityType, PlayerTextures};
use crate::entities::player::weapon_texture::{weapon_texture_entity_to_handle, PivotDisk, Weapon, WeaponTextureEntity, WeaponTextureEntityType, WeaponTextures};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::Vec3;
use bevy::prelude::{Bundle, Circle, ColorMaterial, Commands, Component, Deref, Entity, GlobalTransform, Mesh, Mesh2d, MeshMaterial2d, Name, Res, ResMut, Resource, Sprite, Timer, TimerMode, Transform, Vec2};
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, Friction};
use bevy_rapier2d::prelude::{ActiveEvents, Sensor};
use bevy_renet2::prelude::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct ControlledPlayer;

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
#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(pub Option<Vec2>);

#[derive(Debug, Component)]
pub struct Player(pub f32);

#[derive(Component, Default)]
pub struct Grounded(pub bool);

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
    pub weapon_entity_type: WeaponTextureEntityType,
}

impl PlayerWeaponSelected {
    pub fn default_weapon() -> Self {
        Self {
            weapon_entity_type: WeaponTextureEntityType::Pistol,
        }
    }
}

#[derive(Debug, Component)]
pub struct PlayerNetwork {
    pub id: ClientId,
}

pub fn spawn_player_entity(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &mut Res<PlayerTextures>,
    position: Vec3,
    player_texture_entity_type: &PlayerTextureEntityType,
    id: ClientId,
) -> Entity {
    let rick_texture = PlayerTextureEntity::new(player_texture_entity_type);
    let (image, layout) =
        player_texture_entity_to_handle(&rick_texture.player_texture_entity_type, &mut *texture_atlas_layouts, player_textures);

    commands.spawn((
        Name::new("Player"),
        Sprite::from_atlas_image(
            image,
            TextureAtlas {
                layout,
                index: rick_texture.animation_indices.first as usize,
            },
        ),
        rick_texture.animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player(350.),
        PlayerInput::default(),
        Grounded(false),
        JumpCounter { jumps_left: 2, max_jumps: 2 },
        Transform::from_translation(position).with_scale(Vec3::splat(0.5)),
        GlobalTransform::default(),
        PlayerWeaponSelected::default_weapon(),
        PlayerNetwork { id }
    )).insert(player_texture_entity_type.clone()).id()
}


pub fn player_physics_bundle() -> (RigidBody, LockedAxes, Velocity, Collider, GravityScale, Friction) {
    (
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        Collider::cuboid(59.0, 80.0),
        GravityScale(1.),
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

pub fn spawn_weapon_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    weapon: &mut Res<WeaponTextures>,
    weapon_texture_entity_type: &WeaponTextureEntityType,
) -> (Entity, Entity) {
    let disk_entity = commands.spawn((
        Name::new("PivotDisk"),
        PivotDisk,
        Mesh2d(meshes.add(Mesh::from(Circle::new(40.0)))),
        MeshMaterial2d(materials.add(Color::srgba(0., 0., 0., 0.))),
        Transform::from_xyz(9.5, -31.6, -10.),
        GlobalTransform::default(),
    )).id();


    let weapon_texture = WeaponTextureEntity::new(weapon_texture_entity_type);
    let (image, layout) =
        weapon_texture_entity_to_handle(&weapon_texture.weapon_texture_entity_type, &mut *texture_atlas_layouts, weapon);

    let weapon_entity = commands.spawn((
        Name::new("Weapons"),
        Weapon,
        Sprite::from_atlas_image(
            image,
            TextureAtlas {
                layout,
                index: weapon_texture.animation_indices.first as usize,
            },
        ),
        weapon_texture.animation_indices,
        Transform {
            translation: Vec3::new(52.5, 0.0, 1.0),
            scale: Vec3::splat(1.5),
            ..Default::default()
        },
        GlobalTransform::default(),
    )).id();

    (disk_entity, weapon_entity)
}