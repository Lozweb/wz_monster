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

#[derive(Resource, Clone)]
pub struct WeaponFxTextures(pub HashMap<WeaponFxTextureEntityType, Handle<Image>>);

impl TextureHandleMap<WeaponFxTextureEntityType> for WeaponFxTextures {
    fn get_handle(&self, key: WeaponFxTextureEntityType) -> Option<Handle<Image>> {
        self.0.get(&key).cloned()
    }
}

#[derive(Component, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Default)]
pub enum WeaponFxTextureEntityType {
    #[default]
    Pistol,
    Shotgun,
    Rifle,
    GrenadeLauncher,
}

impl From<&WeaponTextureType> for WeaponFxTextureEntityType {
    fn from(value: &WeaponTextureType) -> Self {
        match value {
            WeaponTextureType::Pistol => WeaponFxTextureEntityType::Pistol,
            WeaponTextureType::Shotgun => WeaponFxTextureEntityType::Shotgun,
            WeaponTextureType::Rifle => WeaponFxTextureEntityType::Rifle,
            WeaponTextureType::GrenadeLauncher => WeaponFxTextureEntityType::GrenadeLauncher,
        }
    }
}
pub struct WeaponFxTextureEntity {
    pub texture_atlas_layout: TextureAtlasLayout,
    pub animation_indices: AnimationIndices,
    pub texture_path: String,
    pub weapon_fx_texture_type: WeaponFxTextureEntityType,
}


const WEAPON_FX_LAYOUT_SIZE: UVec2 = UVec2::new(1300, 3175);
const WEAPON_FX_FRAME_PADDING: u32 = 0;
const WEAPON_FX_FRAME_COUNT: u32 = 1;

pub const PISTOL_FX_SIZE: UVec2 = UVec2::new(31, 15);
const PISTOL_FX_START_MIN: UVec2 = UVec2::new(3, 2431);
impl WeaponFxTextureEntity {
    pub fn new(weapon_texture: &WeaponFxTextureEntityType) -> Self {
        match weapon_texture {
            WeaponFxTextureEntityType::Pistol => make_weapon_fx_texture!(
                WeaponFxTextureEntityType::Pistol,
                PISTOL_FX_START_MIN,
                PISTOL_FX_SIZE
            ),
            WeaponFxTextureEntityType::Shotgun => make_weapon_fx_texture!(
                WeaponFxTextureEntityType::Shotgun,
                PISTOL_FX_START_MIN,
                PISTOL_FX_SIZE
            ),
            WeaponFxTextureEntityType::Rifle => make_weapon_fx_texture!(
                WeaponFxTextureEntityType::Rifle,
                PISTOL_FX_START_MIN,
                PISTOL_FX_SIZE
            ),
            WeaponFxTextureEntityType::GrenadeLauncher => make_weapon_fx_texture!(
                WeaponFxTextureEntityType::GrenadeLauncher,
                PISTOL_FX_START_MIN,
                PISTOL_FX_SIZE
            ),
        }
    }

    pub fn all() -> Vec<(WeaponFxTextureEntityType, &'static str)> {
        vec![
            (WeaponFxTextureEntityType::Pistol, WEAPONS_FX),
            (WeaponFxTextureEntityType::Shotgun, WEAPONS_FX),
            (WeaponFxTextureEntityType::Rifle, WEAPONS_FX),
            (WeaponFxTextureEntityType::GrenadeLauncher, WEAPONS_FX),
        ]
    }
}

impl HasTextureEntityType<WeaponFxTextureEntityType> for WeaponFxTextureEntity {
    fn texture_atlas_layout(&self) -> TextureAtlasLayout {
        self.texture_atlas_layout.clone()
    }

    fn texture_entity_type(&self) -> WeaponFxTextureEntityType {
        self.weapon_fx_texture_type.clone()
    }
}