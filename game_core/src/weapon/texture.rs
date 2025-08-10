use crate::make_weapon_texture;
use crate::player::component::AnimationIndices;
use crate::texture::entity::{HasTextureEntityType, TextureHandleMap};
use crate::texture::system::texture;
use bevy::math::UVec2;
use bevy::prelude::{Component, Handle, Image, Resource, TextureAtlasLayout};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const WEAPONS: &str = "textures/weapons/weapons.png";
#[derive(Resource, Clone)]
pub struct WeaponTextures(pub HashMap<WeaponTextureType, Handle<Image>>);

impl TextureHandleMap<WeaponTextureType> for WeaponTextures {
    fn get_handle(&self, key: WeaponTextureType) -> Option<Handle<Image>> {
        self.0.get(&key).cloned()
    }
}


#[derive(Component, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Default)]
pub enum WeaponTextureType {
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
    pub weapon_texture_type: WeaponTextureType,
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
    pub fn new(weapon_texture: &WeaponTextureType) -> Self {
        match weapon_texture {
            WeaponTextureType::Pistol => make_weapon_texture!(
                WeaponTextureType::Pistol,
                PISTOL_START_MIN,
                PISTOL_SIZE
            ),
            WeaponTextureType::Shotgun => make_weapon_texture!(
                WeaponTextureType::Shotgun,
                SHOTGUN_START_MIN,
                SHOTGUN_SIZE
            ),
            WeaponTextureType::Rifle => make_weapon_texture!(
                WeaponTextureType::Rifle,
                RIFLE_START_MIN,
                RIFLE_SIZE
            ),
            WeaponTextureType::GrenadeLauncher => make_weapon_texture!(
                WeaponTextureType::GrenadeLauncher,
                GRENADE_LAUNCHER_START_MIN,
                GRENADE_LAUNCHER_SIZE
            ),
        }
    }

    pub fn all() -> Vec<(WeaponTextureType, &'static str)> {
        vec![
            (WeaponTextureType::Pistol, WEAPONS),
            (WeaponTextureType::Shotgun, WEAPONS),
            (WeaponTextureType::Rifle, WEAPONS),
            (WeaponTextureType::GrenadeLauncher, WEAPONS),
        ]
    }
}


impl HasTextureEntityType<WeaponTextureType> for WeaponTextureEntity {
    fn texture_atlas_layout(&self) -> TextureAtlasLayout {
        self.texture_atlas_layout.clone()
    }

    fn texture_entity_type(&self) -> WeaponTextureType {
        self.weapon_texture_type.clone()
    }
}