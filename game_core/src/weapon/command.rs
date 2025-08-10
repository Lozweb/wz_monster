use crate::player::component::AnimationTimer;
use crate::texture::entity::TextureHandleMap;
use crate::texture::math::is_face_right;
use crate::texture::system::handle_from_texture;
use crate::weapon::component::{spawn_weapon_fx_physics_bundle, PivotDisk, Weapon};
use crate::weapon::fx_texture::{FxComponent, WeaponFxTextureEntity, WeaponFxTextureEntityType, WeaponFxTextures};
use crate::weapon::texture::{WeaponTextureEntity, WeaponTextureType, WeaponTextures};
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::image::{Image, TextureAtlas, TextureAtlasLayout};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Circle, ColorMaterial, Commands, Entity, GlobalTransform, Mesh, Mesh2d, MeshMaterial2d, Name, Res, ResMut, Sprite, Timer, TimerMode, Transform};
use bevy_renet2::prelude::ClientId;

pub fn spawn_weapon_fx(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    weapon_fx_textures: &mut Res<WeaponFxTextures>,
    position: Vec3,
    weapon_fx_texture_entity_type: &WeaponFxTextureEntityType,
    aim_direction: f32,
    client_id: ClientId,
) -> Entity {
    let weapon_fx_texture = WeaponFxTextureEntity::new(weapon_fx_texture_entity_type);
    let (image, layout) =
        weapon_texture_fx_entity_to_handle(&weapon_fx_texture.weapon_fx_texture_type, &mut *texture_atlas_layouts, weapon_fx_textures);
    let is_face_right = is_face_right(aim_direction);

    commands.spawn((
        FxComponent,
        Sprite {
            flip_y: !is_face_right,
            ..Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout,
                    index: weapon_fx_texture.animation_indices.first as usize,
                },
            )
        },
        weapon_fx_texture.animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Transform::from_translation(position)
            .with_scale(Vec3::splat(1.))
            .with_rotation(Quat::from_rotation_z(aim_direction)),
        GlobalTransform::default(),
    )).insert(spawn_weapon_fx_physics_bundle(aim_direction, client_id)).id()
}

pub fn spawn_weapon_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    weapon: &mut Res<WeaponTextures>,
    weapon_texture_entity_type: &WeaponTextureType,
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
        weapon_texture_entity_to_handle(&weapon_texture.weapon_texture_type, &mut *texture_atlas_layouts, weapon);

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
pub fn weapon_texture_entity_to_handle(
    weapon_texture_type: &WeaponTextureType,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    weapon_textures: &Res<WeaponTextures>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    handle_from_texture(
        weapon_texture_type,
        texture_atlas_layouts,
        weapon_textures,
        WeaponTextureEntity::new,
        WeaponTextures::get_handle,
    )
}
pub fn weapon_texture_fx_entity_to_handle(
    weapon_fx_texture_type: &WeaponFxTextureEntityType,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    weapon_textures: &Res<WeaponFxTextures>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    handle_from_texture(
        weapon_fx_texture_type,
        texture_atlas_layouts,
        weapon_textures,
        WeaponFxTextureEntity::new,
        WeaponFxTextures::get_handle,
    )
}
