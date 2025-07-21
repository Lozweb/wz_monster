use crate::entities::player::component::AnimationIndices;
use bevy::asset::Handle;
use bevy::image::{Image, TextureAtlasLayout};
use bevy::log::error;
use bevy::math::{URect, UVec2};
use bevy::platform::collections::HashMap;
use bevy::prelude::{AssetServer, Assets, Commands, Component, Res, ResMut, Resource};
use serde::{Deserialize, Serialize};

pub const RICK_1: &str = "textures/rick1.png";
pub const RICK_2: &str = "textures/rick2.png";

#[derive(Resource, Clone)]
pub struct PlayerTextures(pub HashMap<TextureEntityType, Handle<Image>>);

impl PlayerTextures {
    pub fn get_handle(&self, key: TextureEntityType) -> Option<Handle<Image>> {
        self.0.get(&key).cloned()
    }
}

#[derive(Component, Clone, Eq, Hash, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum TextureEntityType {
    #[default]
    Rick1,
    Rick2,
}

pub struct TextureEntity {
    pub texture_atlas_layout: TextureAtlasLayout,
    pub animation_indices: AnimationIndices,
    pub texture_path: String,
    pub player_texture: TextureEntityType,
}

impl TextureEntity {
    pub fn new(player_texture: &TextureEntityType) -> Self {
        match player_texture {
            TextureEntityType::Rick1 => {
                let layout_size = UVec2::new(1036, 740);
                let start_min = UVec2::new(519, 173);
                texture(layout_size, start_min, RICK_1, player_texture.clone())
            }
            TextureEntityType::Rick2 => {
                let layout_size = UVec2::new(816, 1241);
                let start_min = UVec2::new(5, 908);
                texture(layout_size, start_min, RICK_2, player_texture.clone())
            }
        }
    }

    pub fn all() -> Vec<(TextureEntityType, &'static str)> {
        vec![
            (TextureEntityType::Rick1, RICK_1),
            (TextureEntityType::Rick2, RICK_2),
        ]
    }
}
const PADDING: u32 = 14;
const FRAME_SIZE: UVec2 = UVec2::new(117, 160);
const FRAME_COUNT: usize = 4;
fn generate_frames(start_min: UVec2) -> Vec<URect> {
    (0..FRAME_COUNT)
        .map(|i| {
            let offset_x = i as u32 * (FRAME_SIZE.x + PADDING);
            let min = UVec2::new(start_min.x + offset_x, start_min.y);
            let max = UVec2::new(min.x + FRAME_SIZE.x, min.y + FRAME_SIZE.y);
            URect { min, max }
        })
        .collect()
}

fn texture(layout_size: UVec2, start_min: UVec2, texture_path: &str, texture_entity_type: TextureEntityType) -> TextureEntity {
    let frames = generate_frames(start_min);

    let mut layout = TextureAtlasLayout::new_empty(layout_size);
    for frame in &frames {
        layout.add_texture(*frame);
    }

    TextureEntity {
        texture_atlas_layout: layout,
        animation_indices: AnimationIndices { first: 0, last: 3 },
        texture_path: texture_path.to_string(),
        player_texture: texture_entity_type,
    }
}

pub fn texture_entity_to_handle(
    texture_entity_type: &TextureEntityType,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player_textures: &Res<PlayerTextures>,
) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
    let texture_entity = TextureEntity::new(&texture_entity_type.clone());
    let texture_atlas_layout = texture_atlas_layouts.add(texture_entity.texture_atlas_layout);
    let Some(image) = player_textures.get_handle(texture_entity.player_texture) else {
        error!("Failed to get player texture for");
        return (Handle::default(), Handle::default());
    };

    (image, texture_atlas_layout)
}

pub fn player_textures_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_textures = PlayerTextures(
        TextureEntity::all()
            .into_iter()
            .map(|(texture_type, path)| (texture_type, asset_server.load(path)))
            .collect()
    );

    commands.insert_resource(player_textures);
}
