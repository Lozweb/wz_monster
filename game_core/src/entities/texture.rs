use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::{Image, TextureAtlasLayout};
use bevy::log::error;
use bevy::prelude::{Commands, Res, ResMut, URect, UVec2};
use std::collections::HashMap;

pub fn generate_frames(start_min: UVec2, frame_count: u32, frame_size: UVec2, padding: u32) -> Vec<URect> {
    (0..frame_count)
        .map(|i| {
            let offset_x = i * (frame_size.x + padding);
            let min = UVec2::new(start_min.x + offset_x, start_min.y);
            let max = UVec2::new(min.x + frame_size.x, min.y + frame_size.y);
            URect { min, max }
        })
        .collect()
}

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

pub trait HasTextureEntityType<T> {
    fn texture_atlas_layout(&self) -> TextureAtlasLayout;
    fn texture_entity_type(&self) -> T;
}

pub fn texture_entity_to_handle<T, R, E>(
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

pub trait TextureHandleMap<K> {
    fn get_handle(&self, key: K) -> Option<Handle<Image>>;
}

#[macro_export]
macro_rules! make_weapon_texture {
    ($weapon_type:expr, $start_min:expr, $frame_size:expr) => {
        texture(
            WEAPON_LAYOUT_SIZE,
            $start_min,
            WEAPONS,
            $weapon_type,
            WEAPON_FRAME_COUNT,
            $frame_size,
            WEAPON_FRAME_PADDING,
            |layout, typ, path| WeaponTextureEntity {
                texture_atlas_layout: layout,
                animation_indices: AnimationIndices { first: 0, last: WEAPON_FRAME_COUNT - 1 },
                texture_path: path,
                weapon_texture_entity_type: typ,
            }
        )
    };
}

#[macro_export]
macro_rules! make_player_texture {
    ($weapon_type:expr, $layout_size:expr, $path:expr, $start_min:expr, $frame_size:expr) => {
        texture(
            $layout_size,
            $start_min,
            $path,
            $weapon_type,
            PLAYER_FRAME_COUNT,
            $frame_size,
            PLAYER_FRAME_PADDING,
            |layout, typ, path| PlayerTextureEntity {
                texture_atlas_layout: layout,
                animation_indices: AnimationIndices { first: 0, last: PLAYER_FRAME_COUNT - 1 },
                texture_path: path,
                player_texture_entity_type: typ,
            }
        )
    };
}

#[macro_export]
macro_rules! make_weapon_fx_texture {
    ($weapon_type:expr, $start_min:expr, $frame_size:expr) => {
        texture(
            WEAPON_FX_LAYOUT_SIZE,
            $start_min,
            WEAPONS_FX,
            $weapon_type,
            WEAPON_FX_FRAME_COUNT,
            $frame_size,
            WEAPON_FX_FRAME_PADDING,
            |layout, typ, path| WeaponFxTextureEntity {
                texture_atlas_layout: layout,
                animation_indices: AnimationIndices { first: 0, last: WEAPON_FX_FRAME_COUNT - 1 },
                texture_path: path,
                weapon_fx_texture_entity_type: typ,
            }
        )
    };
}