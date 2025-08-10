use crate::player::texture::{PlayerTextureEntity, PlayerTextures};
use crate::texture::entity::HasTextureEntityType;
use crate::texture::frame::generate_frames;
use crate::weapon::fx_texture::{WeaponFxTextureEntity, WeaponFxTextures};
use crate::weapon::texture::{WeaponTextureEntity, WeaponTextures};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::{Image, TextureAtlasLayout};
use bevy::log::error;
use bevy::math::UVec2;
use bevy::prelude::{Commands, Res, ResMut};
use std::collections::HashMap;

#[allow(clippy::too_many_arguments)]
pub fn texture<T, U>(
    layout_size: UVec2,
    start_min: UVec2,
    texture_path: &str,
    texture_entity_type: T,
    frame_count: u32,
    frame_size: UVec2,
    padding: u32,
    make_entity: impl Fn(TextureAtlasLayout, T, String) -> U,
) -> U {
    let frames = generate_frames(start_min, frame_count, frame_size, padding);

    let mut layout = TextureAtlasLayout::new_empty(layout_size);
    for frame in &frames {
        layout.add_texture(*frame);
    }

    make_entity(layout, texture_entity_type, texture_path.to_string())
}
pub fn load_textures<K, R>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    all: fn() -> Vec<(K, &'static str)>,
    texture_constructor: fn(HashMap<K, Handle<Image>>) -> R,
)
where
    K: Eq + std::hash::Hash + Clone,
    R: bevy::prelude::Resource,
{
    let textures = texture_constructor(
        all()
            .into_iter()
            .map(|(texture_type, path)| (texture_type, asset_server.load(path)))
            .collect::<HashMap<K, Handle<Image>>>(),
    );
    commands.insert_resource(textures);
}

pub fn handle_from_texture<T, R, E>(
    texture_entity_type: &T,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    textures: &Res<R>,
    entity_constructor: fn(&T) -> E,
    get_handle: fn(&R, T) -> Option<Handle<Image>>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>)
where
    T: Clone + std::fmt::Debug,
    E: HasTextureEntityType<T>,
    R: bevy::prelude::Resource,
{
    let texture_entity = entity_constructor(texture_entity_type);
    let texture_atlas_layout = texture_atlas_layouts.add(texture_entity.texture_atlas_layout());
    let Some(image) = get_handle(textures, texture_entity.texture_entity_type()) else {
        error!("Failed to get texture for {:?}", texture_entity_type);
        return (Handle::default(), Handle::default());
    };
    (image, texture_atlas_layout)
}

pub fn load_player_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_textures(
        &mut commands,
        &asset_server,
        PlayerTextureEntity::all,
        PlayerTextures,
    );
}

pub fn load_weapon_fx_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_textures(
        &mut commands,
        &asset_server,
        WeaponFxTextureEntity::all,
        WeaponFxTextures,
    );
}

pub fn load_weapon_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_textures(
        &mut commands,
        &asset_server,
        WeaponTextureEntity::all,
        WeaponTextures,
    );
}