use crate::entities::player::component::AnimationIndices;
use crate::entities::texture::texture;
use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlasLayout};
use bevy::log::error;
use bevy::math::UVec2;
use bevy::platform::collections::HashMap;
use bevy::prelude::{AssetServer, Assets, Commands, Component, Res, ResMut, Resource};
use serde::{Deserialize, Serialize};

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
const PLAYER_FRAME_PADDING: u32 = 14;
const PLAYER_FRAME_SIZE: UVec2 = UVec2::new(117, 160);
const PLAYER_FRAME_COUNT: u32 = 4;

impl PlayerTextureEntity {
    pub fn new(player_texture: &PlayerTextureEntityType) -> Self {
        match player_texture {
            PlayerTextureEntityType::Rick1 => {
                Self::make_player_texture(
                    UVec2::new(1036, 740),
                    UVec2::new(519, 173),
                    PlayerTextureEntityType::Rick1,
                )
            }
            PlayerTextureEntityType::Rick2 => {
                Self::make_player_texture(
                    UVec2::new(816, 1241),
                    UVec2::new(5, 908),
                    PlayerTextureEntityType::Rick2,
                )
            }
        }
    }

    fn make_player_texture(
        layout_size: UVec2,
        start_min: UVec2,
        player_texture_entity_type: PlayerTextureEntityType,
    ) -> PlayerTextureEntity {
        texture(
            layout_size,
            start_min,
            RICK_2,
            player_texture_entity_type,
            PLAYER_FRAME_COUNT,
            PLAYER_FRAME_SIZE,
            PLAYER_FRAME_PADDING,
            |layout, player_texture, texture_path| PlayerTextureEntity {
                texture_atlas_layout: layout,
                animation_indices: AnimationIndices { first: 0, last: 0 },
                texture_path,
                player_texture_entity_type: player_texture,
            },
        )
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

pub fn player_texture_entity_to_handle(
    texture_entity_type: &PlayerTextureEntityType,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &Res<PlayerTextures>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    let texture_entity = PlayerTextureEntity::new(&texture_entity_type.clone());
    let texture_atlas_layout = texture_atlas_layouts.add(texture_entity.texture_atlas_layout);
    let Some(image) = player_textures.get_handle(texture_entity.player_texture_entity_type) else {
        error!("Failed to get player texture for");
        return (Handle::default(), Handle::default());
    };

    (image, texture_atlas_layout)
}

pub fn player_textures_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_textures = PlayerTextures(
        PlayerTextureEntity::all()
            .into_iter()
            .map(|(texture_type, path)| (texture_type, asset_server.load(path)))
            .collect()
    );

    commands.insert_resource(player_textures);
}
