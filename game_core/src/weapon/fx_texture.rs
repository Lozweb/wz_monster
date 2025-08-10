use crate::make_weapon_fx_texture;
use crate::player::component::AnimationIndices;
use crate::texture::entity::{HasTextureEntityType, TextureHandleMap};
use crate::texture::system::texture;
use crate::weapon::texture::WeaponTextureType;
use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{Component, Resource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const WEAPONS_FX: &str = "textures/weapons/weapons_fx.png";

#[derive(Component)]
pub struct FxComponent;

#[derive(Resource, Clone, Debug)]
pub struct WeaponFxTextures(pub HashMap<WeaponFxTextureType, Handle<Image>>);

impl TextureHandleMap<WeaponFxTextureType> for WeaponFxTextures {
    fn get_handle(&self, key: WeaponFxTextureType) -> Option<Handle<Image>> {
        self.0.get(&key).cloned()
    }
}

#[derive(Component, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Default)]
pub enum WeaponFxTextureType {
    #[default]
    Pistol,
    Shotgun,
    Rifle,
    GrenadeLauncher,
}

impl From<&WeaponTextureType> for WeaponFxTextureType {
    fn from(value: &WeaponTextureType) -> Self {
        match value {
            WeaponTextureType::Pistol => WeaponFxTextureType::Pistol,
            WeaponTextureType::Shotgun => WeaponFxTextureType::Shotgun,
            WeaponTextureType::Rifle => WeaponFxTextureType::Rifle,
            WeaponTextureType::GrenadeLauncher => WeaponFxTextureType::GrenadeLauncher,
        }
    }
}
pub struct WeaponFxTextureEntity {
    pub texture_atlas_layout: TextureAtlasLayout,
    pub animation_indices: AnimationIndices,
    pub texture_path: String,
    pub weapon_fx_texture_type: WeaponFxTextureType,
}


const WEAPON_FX_LAYOUT_SIZE: UVec2 = UVec2::new(1300, 3175);
const WEAPON_FX_FRAME_PADDING: u32 = 0;
const WEAPON_FX_FRAME_COUNT: u32 = 1;

pub const PISTOL_FX_SIZE: UVec2 = UVec2::new(31, 15);
const PISTOL_FX_START_MIN: UVec2 = UVec2::new(3, 2431);
impl WeaponFxTextureEntity {
    pub fn new(weapon_texture: &WeaponFxTextureType) -> Self {
        match weapon_texture {
            WeaponFxTextureType::Pistol => make_weapon_fx_texture!(
                WeaponFxTextureType::Pistol,
                PISTOL_FX_START_MIN,
                PISTOL_FX_SIZE
            ),
            WeaponFxTextureType::Shotgun => make_weapon_fx_texture!(
                WeaponFxTextureType::Shotgun,
                PISTOL_FX_START_MIN,
                PISTOL_FX_SIZE
            ),
            WeaponFxTextureType::Rifle => make_weapon_fx_texture!(
                WeaponFxTextureType::Rifle,
                PISTOL_FX_START_MIN,
                PISTOL_FX_SIZE
            ),
            WeaponFxTextureType::GrenadeLauncher => make_weapon_fx_texture!(
                WeaponFxTextureType::GrenadeLauncher,
                PISTOL_FX_START_MIN,
                PISTOL_FX_SIZE
            ),
        }
    }

    pub fn all() -> Vec<(WeaponFxTextureType, &'static str)> {
        vec![
            (WeaponFxTextureType::Pistol, WEAPONS_FX),
            (WeaponFxTextureType::Shotgun, WEAPONS_FX),
            (WeaponFxTextureType::Rifle, WEAPONS_FX),
            (WeaponFxTextureType::GrenadeLauncher, WEAPONS_FX),
        ]
    }
}

impl HasTextureEntityType<WeaponFxTextureType> for WeaponFxTextureEntity {
    fn texture_atlas_layout(&self) -> TextureAtlasLayout {
        self.texture_atlas_layout.clone()
    }

    fn texture_entity_type(&self) -> WeaponFxTextureType {
        self.weapon_fx_texture_type.clone()
    }
}