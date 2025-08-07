use crate::entities::player::component::AnimationIndices;
use crate::entities::texture::texture;
use crate::entities::texture::{load_textures, texture_entity_to_handle, HasTextureEntityType};
use crate::make_player_texture;
use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlasLayout};
use bevy::math::UVec2;
use bevy::prelude::{AssetServer, Assets, Commands, Component, Res, ResMut, Resource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const RICK_1: &str = "textures/players/rick1.png";
pub const RICK_2: &str = "textures/players/rick2.png";

#[derive(Resource, Clone)]
pub struct PlayerTextures(pub HashMap<PlayerTextureEntityType, Handle<Image>>);

impl PlayerTextures {
    pub fn get_handle(&self, key: PlayerTextureEntityType) -> Option<Handle<Image>> {
        self.0.get(&key).cloned()
    }
}

#[derive(Component, Clone, Eq, Hash, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum PlayerTextureEntityType {
    #[default]
    Rick1,
    Rick2,
}

pub struct PlayerTextureEntity {
    pub texture_atlas_layout: TextureAtlasLayout,
    pub animation_indices: AnimationIndices,
    pub texture_path: String,
    pub player_texture_entity_type: PlayerTextureEntityType,
}

const RICK_1_LAYOUT_SIZE: UVec2 = UVec2::new(1036, 740);
const RICK_2_LAYOUT_SIZE: UVec2 = UVec2::new(816, 1241);
const PLAYER_FRAME_PADDING: u32 = 14;
const PLAYER_FRAME_SIZE: UVec2 = UVec2::new(117, 160);
const PLAYER_FRAME_COUNT: u32 = 4;

const RICK_1_START_MIN: UVec2 = UVec2::new(519, 173);
const RICK_2_START_MIN: UVec2 = UVec2::new(5, 908);

impl PlayerTextureEntity {
    pub fn new(player_texture: &PlayerTextureEntityType) -> Self {
        match player_texture {
            PlayerTextureEntityType::Rick1 => make_player_texture!(
                PlayerTextureEntityType::Rick1,
                RICK_1_LAYOUT_SIZE,
                RICK_1,
                RICK_1_START_MIN,
                PLAYER_FRAME_SIZE
            ),

            PlayerTextureEntityType::Rick2 => make_player_texture!(
                PlayerTextureEntityType::Rick2,
                RICK_2_LAYOUT_SIZE,
                RICK_2,
                RICK_2_START_MIN,
                PLAYER_FRAME_SIZE
            ),
        }
    }

    pub fn all() -> Vec<(PlayerTextureEntityType, &'static str)> {
        vec![
            (PlayerTextureEntityType::Rick1, RICK_1),
            (PlayerTextureEntityType::Rick2, RICK_2),
        ]
    }
}

pub fn rand_player_texture_entity_type() -> PlayerTextureEntityType {
    if fastrand::bool() {
        PlayerTextureEntityType::Rick1
    } else {
        PlayerTextureEntityType::Rick2
    }
}
impl HasTextureEntityType<PlayerTextureEntityType> for PlayerTextureEntity {
    fn texture_atlas_layout(&self) -> TextureAtlasLayout {
        self.texture_atlas_layout.clone()
    }
    fn texture_entity_type(&self) -> PlayerTextureEntityType {
        self.player_texture_entity_type.clone()
    }
}

pub fn player_texture_entity_to_handle(
    texture_entity_type: &PlayerTextureEntityType,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &Res<PlayerTextures>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    texture_entity_to_handle(
        texture_entity_type,
        texture_atlas_layouts,
        player_textures,
        PlayerTextureEntity::new,
        PlayerTextures::get_handle,
    )
}

pub fn load_player_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_textures(
        &mut commands,
        &asset_server,
        PlayerTextureEntity::all,
        PlayerTextures,
    );
}