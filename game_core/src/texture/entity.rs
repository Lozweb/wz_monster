use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlasLayout};

pub trait TextureHandleMap<K> {
    fn get_handle(&self, key: K) -> Option<Handle<Image>>;
}
pub trait HasTextureEntityType<T> {
    fn texture_atlas_layout(&self) -> TextureAtlasLayout;
    fn texture_entity_type(&self) -> T;
}
