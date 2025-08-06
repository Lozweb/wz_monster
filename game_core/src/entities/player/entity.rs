use crate::entities::player::component::{AnimationTimer, Grounded, JumpCounter, Player, PlayerInput, PlayerNetwork, PlayerWeaponSelected};
use crate::entities::player::texture::{player_texture_entity_to_handle, PlayerTextureEntity, PlayerTextureEntityType, PlayerTextures};
use bevy::image::{TextureAtlas, TextureAtlasLayout};
use bevy::math::Vec3;
use bevy::prelude::{Assets, Commands, Entity, GlobalTransform, Name, Res, ResMut, Sprite, Timer, TimerMode, Transform};
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Friction, Sensor};
use bevy_renet2::prelude::ClientId;

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
                index: rick_texture.animation_indices.first,
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