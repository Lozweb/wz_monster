use crate::entities::player::component::AnimationIndices;
use bevy::asset::Assets;
use bevy::math::UVec2;
use bevy::prelude::{error, AssetServer, Commands, Component, Handle, Image, Res, ResMut, Resource, TextureAtlasLayout};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const WEAPONS: &str = "textures/weapons/weapons.png";

#[derive(Resource, Clone)]
pub struct WeaponTextures(pub HashMap<WeaponTextureEntityType, Handle<Image>>);

impl WeaponTextures {
    pub fn get_handle(&self, key: WeaponTextureEntityType) -> Option<Handle<Image>> {
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
const SHOTGUN_SIZE: UVec2 = UVec2::new(51, 23);
const RIFLE_SIZE: UVec2 = UVec2::new(51, 23);
const GRENADE_LAUNCHER_SIZE: UVec2 = UVec2::new(51, 23);
impl WeaponTextureEntity {
    pub fn new(weapon_texture: &WeaponTextureEntityType) -> Self {
        match weapon_texture {
            WeaponTextureEntityType::Pistol => {
                Self::make_weapon_texture(
                    WEAPON_LAYOUT_SIZE,
                    UVec2::new(358, 200),
                    WEAPON_FRAME_COUNT,
                    PISTOL_SIZE,
                    WeaponTextureEntityType::Shotgun,
                )
            }
            WeaponTextureEntityType::Shotgun => {
                Self::make_weapon_texture(
                    WEAPON_LAYOUT_SIZE,
                    UVec2::new(358, 200),
                    WEAPON_FRAME_COUNT,
                    SHOTGUN_SIZE,
                    WeaponTextureEntityType::Pistol,
                )
            }
            WeaponTextureEntityType::Rifle => {
                Self::make_weapon_texture(
                    WEAPON_LAYOUT_SIZE,
                    UVec2::new(358, 200),
                    WEAPON_FRAME_COUNT,
                    RIFLE_SIZE,
                    WeaponTextureEntityType::Rifle,
                )
            }
            WeaponTextureEntityType::GrenadeLauncher => {
                Self::make_weapon_texture(
                    WEAPON_LAYOUT_SIZE,
                    UVec2::new(358, 200),
                    WEAPON_FRAME_COUNT,
                    GRENADE_LAUNCHER_SIZE,
                    WeaponTextureEntityType::GrenadeLauncher,
                )
            }
        }
    }

    fn make_weapon_texture(
        layout_size: UVec2,
        start_min: UVec2,
        frame_count: u32,
        frame_size: UVec2,
        player_texture_entity_type: WeaponTextureEntityType,
    ) -> WeaponTextureEntity {
        crate::entities::texture::texture(
            layout_size,
            start_min,
            WEAPONS,
            player_texture_entity_type,
            frame_count,
            frame_size,
            WEAPON_FRAME_PADDING,
            |layout, weapon_texture, texture_path| WeaponTextureEntity {
                texture_atlas_layout: layout,
                animation_indices: AnimationIndices { first: 0, last: 0 },
                texture_path,
                weapon_texture_entity_type: weapon_texture,
            },
        )
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

pub fn weapon_texture_entity_to_handle(
    weapon_texture_type: &WeaponTextureEntityType,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    weapon_textures: &Res<WeaponTextures>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    let texture_entity = WeaponTextureEntity::new(weapon_texture_type);
    let texture_atlas_layout = texture_atlas_layouts.add(texture_entity.texture_atlas_layout);
    let Some(image) = weapon_textures.get_handle(texture_entity.weapon_texture_entity_type) else {
        error!("Weapon texture not found: {:?}", weapon_texture_type);
        return (Handle::default(), Handle::default());
    };
    (image, texture_atlas_layout)
}

pub fn weapon_texture_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapon_textures = WeaponTextures(
        WeaponTextureEntity::all()
            .into_iter()
            .map(|(texture_type, path)| (texture_type, asset_server.load(path)))
            .collect(),
    );
    commands.insert_resource(weapon_textures);
}