use crate::player::component::AnimationIndices;
use crate::texture::entity::HasTextureEntityType;
use crate::texture::system::texture;

use crate::make_player_texture;
use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{Component, Resource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const RICK_1: &str = "textures/players/rick1.png";
pub const RICK_2: &str = "textures/players/rick2.png";

#[derive(Resource, Clone)]
pub struct PlayerTextures(pub HashMap<PlayerTextureType, Handle<Image>>);

impl PlayerTextures {
    pub fn get_handle(&self, key: PlayerTextureType) -> Option<Handle<Image>> {
        self.0.get(&key).cloned()
    }
}

#[derive(Component, Clone, Eq, Hash, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum PlayerTextureType {
    #[default]
    Rick1,
    Rick2,
}

pub struct PlayerTextureEntity {
    pub texture_atlas_layout: TextureAtlasLayout,
    pub animation_indices: AnimationIndices,
    pub texture_path: String,
    pub player_texture_type: PlayerTextureType,
}

const RICK_1_LAYOUT_SIZE: UVec2 = UVec2::new(1036, 740);
const RICK_2_LAYOUT_SIZE: UVec2 = UVec2::new(816, 1241);
const PLAYER_FRAME_PADDING: u32 = 14;
const PLAYER_FRAME_SIZE: UVec2 = UVec2::new(117, 160);
const PLAYER_FRAME_COUNT: u32 = 4;

const RICK_1_START_MIN: UVec2 = UVec2::new(519, 173);
const RICK_2_START_MIN: UVec2 = UVec2::new(5, 908);

impl PlayerTextureEntity {
    pub fn new(player_texture: &PlayerTextureType) -> Self {
        match player_texture {
            PlayerTextureType::Rick1 => make_player_texture!(
                PlayerTextureType::Rick1,
                RICK_1_LAYOUT_SIZE,
                RICK_1,
                RICK_1_START_MIN,
                PLAYER_FRAME_SIZE
            ),

            PlayerTextureType::Rick2 => make_player_texture!(
                PlayerTextureType::Rick2,
                RICK_2_LAYOUT_SIZE,
                RICK_2,
                RICK_2_START_MIN,
                PLAYER_FRAME_SIZE
            ),
        }
    }

    pub fn all() -> Vec<(PlayerTextureType, &'static str)> {
        vec![
            (PlayerTextureType::Rick1, RICK_1),
            (PlayerTextureType::Rick2, RICK_2),
        ]
    }
}


impl HasTextureEntityType<PlayerTextureType> for PlayerTextureEntity {
    fn texture_atlas_layout(&self) -> TextureAtlasLayout {
        self.texture_atlas_layout.clone()
    }
    fn texture_entity_type(&self) -> PlayerTextureType {
        self.player_texture_type.clone()
    }
}


