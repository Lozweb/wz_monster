use crate::player::component::{AnimationTimer, Grounded, JumpCounter, Player, PlayerChildren, PlayerInput, PlayerNetwork, PlayerWeaponSelected};
use crate::player::texture::{PlayerTextureEntity, PlayerTextureType, PlayerTextures};
use crate::texture::system::handle_from_texture;
use bevy::asset::{Assets, Handle};
use bevy::image::{Image, TextureAtlas, TextureAtlasLayout};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Entity, GlobalTransform, Name, Res, ResMut, Sprite, Timer, TimerMode, Transform};
use bevy_renet2::prelude::ClientId;
pub struct SpawnPlayerParams<'a> {
    pub pivot: Entity,
    pub weapon: Entity,
    pub sensor: Option<Entity>,
    pub position: Vec3,
    pub player_texture_type: &'a PlayerTextureType,
    pub client_id: ClientId,
}
pub fn spawn_player_entity(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &mut Res<PlayerTextures>,
    SpawnPlayerParams {
        pivot,
        weapon,
        sensor,
        position,
        player_texture_type,
        client_id,
    }: SpawnPlayerParams,
) -> Entity {
    let rick_texture = PlayerTextureEntity::new(player_texture_type);
    let (image, layout) =
        handle_from_player_texture(
            &rick_texture.player_texture_type,
            &mut *texture_atlas_layouts, player_textures,
        );

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
        Player { speed: 350. },
        PlayerInput::default(),
        Grounded(false),
        JumpCounter { jumps_left: 2, max_jumps: 2 },
        Transform::from_translation(position).with_scale(Vec3::splat(0.5)),
        GlobalTransform::default(),
        PlayerWeaponSelected::default_weapon(),
        PlayerNetwork { id: client_id },
        PlayerChildren { pivot, weapon, sensor }
    )).insert(player_texture_type.clone()).id()
}
pub fn handle_from_player_texture(
    texture_entity_type: &PlayerTextureType,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &Res<PlayerTextures>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    handle_from_texture(
        texture_entity_type,
        texture_atlas_layouts,
        player_textures,
        PlayerTextureEntity::new,
        PlayerTextures::get_handle,
    )
}
pub fn rand_player_texture_entity_type() -> PlayerTextureType {
    if fastrand::bool() {
        PlayerTextureType::Rick1
    } else {
        PlayerTextureType::Rick2
    }
}