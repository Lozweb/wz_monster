use crate::entities::player::component::AnimationIndices;
// Rust
use crate::entities::texture::texture;
use crate::entities::texture::{load_textures, texture_entity_to_handle, HasTextureEntityType, TextureHandleMap};
use crate::make_weapon_texture;
use bevy::asset::Assets;
use bevy::math::UVec2;
use bevy::prelude::{AssetServer, Commands, Component, Handle, Image, Res, ResMut, Resource, TextureAtlasLayout};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component, Debug, Clone, Default)]
pub struct Weapon;

#[derive(Component, Debug, Clone, Default)]
pub struct PivotDisk;

pub const WEAPONS: &str = "textures/weapons/weapons.png";

#[derive(Resource, Clone)]
pub struct WeaponTextures(pub HashMap<WeaponTextureEntityType, Handle<Image>>);

impl TextureHandleMap<WeaponTextureEntityType> for WeaponTextures {
    fn get_handle(&self, key: WeaponTextureEntityType) -> Option<Handle<Image>> {
        self.0.get(&key).cloned()
    }
}


#[derive(Component, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Default)]
pub enum WeaponTextureEntityType {
    #[default]
    Pistol,
    Shotgun,
    Rifle,
    GrenadeLauncher,
}


pub struct WeaponTextureEntity {
    pub texture_atlas_layout: TextureAtlasLayout,
    pub animation_indices: AnimationIndices,
    pub texture_path: String,
    pub weapon_texture_entity_type: WeaponTextureEntityType,
}


const WEAPON_LAYOUT_SIZE: UVec2 = UVec2::new(502, 448);
const WEAPON_FRAME_PADDING: u32 = 0;
const WEAPON_FRAME_COUNT: u32 = 1;

const PISTOL_SIZE: UVec2 = UVec2::new(51, 23);
const PISTOL_START_MIN: UVec2 = UVec2::new(358, 200);

const SHOTGUN_SIZE: UVec2 = UVec2::new(51, 23);
const SHOTGUN_START_MIN: UVec2 = UVec2::new(358, 200);

const RIFLE_SIZE: UVec2 = UVec2::new(51, 23);
const RIFLE_START_MIN: UVec2 = UVec2::new(358, 200);

const GRENADE_LAUNCHER_SIZE: UVec2 = UVec2::new(51, 23);
const GRENADE_LAUNCHER_START_MIN: UVec2 = UVec2::new(358, 200);


impl WeaponTextureEntity {
    pub fn new(weapon_texture: &WeaponTextureEntityType) -> Self {
        match weapon_texture {
            WeaponTextureEntityType::Pistol => make_weapon_texture!(
                WeaponTextureEntityType::Pistol,
                PISTOL_START_MIN,
                PISTOL_SIZE
            ),
            WeaponTextureEntityType::Shotgun => make_weapon_texture!(
                WeaponTextureEntityType::Shotgun,
                SHOTGUN_START_MIN,
                SHOTGUN_SIZE
            ),
            WeaponTextureEntityType::Rifle => make_weapon_texture!(
                WeaponTextureEntityType::Rifle,
                RIFLE_START_MIN,
                RIFLE_SIZE
            ),
            WeaponTextureEntityType::GrenadeLauncher => make_weapon_texture!(
                WeaponTextureEntityType::GrenadeLauncher,
                GRENADE_LAUNCHER_START_MIN,
                GRENADE_LAUNCHER_SIZE
            ),
        }
    }

    pub fn all() -> Vec<(WeaponTextureEntityType, &'static str)> {
        vec![
            (WeaponTextureEntityType::Pistol, WEAPONS),
            (WeaponTextureEntityType::Shotgun, WEAPONS),
            (WeaponTextureEntityType::Rifle, WEAPONS),
            (WeaponTextureEntityType::GrenadeLauncher, WEAPONS),
        ]
    }
}


impl HasTextureEntityType<WeaponTextureEntityType> for WeaponTextureEntity {
    fn texture_atlas_layout(&self) -> TextureAtlasLayout {
        self.texture_atlas_layout.clone()
    }

    fn texture_entity_type(&self) -> WeaponTextureEntityType {
        self.weapon_texture_entity_type.clone()
    }
}

pub fn weapon_texture_entity_to_handle(
    weapon_texture_type: &WeaponTextureEntityType,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    weapon_textures: &Res<WeaponTextures>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    texture_entity_to_handle(
        weapon_texture_type,
        texture_atlas_layouts,
        weapon_textures,
        WeaponTextureEntity::new,
        WeaponTextures::get_handle,
    )
}


pub fn load_weapon_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_textures(
        &mut commands,
        &asset_server,
        WeaponTextureEntity::all,
        WeaponTextures,
    );
}