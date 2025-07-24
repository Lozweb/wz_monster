use crate::entities::weapons::component::{PivotDisk, Weapon};
use crate::entities::weapons::texture::{weapon_texture_entity_to_handle, WeaponTextureEntity, WeaponTextureEntityType, WeaponTextures};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::image::TextureAtlas;
use bevy::math::Vec3;
use bevy::prelude::{Circle, ColorMaterial, Commands, Entity, GlobalTransform, Mesh, Mesh2d, MeshMaterial2d, Name, Res, ResMut, Sprite, TextureAtlasLayout, Transform};

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
        Transform::from_xyz(9.5, -31.6, 0.),
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
                index: weapon_texture.animation_indices.first,
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