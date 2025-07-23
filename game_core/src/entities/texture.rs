use bevy::image::TextureAtlasLayout;
use bevy::prelude::{URect, UVec2};

pub fn generate_frames(start_min: UVec2, frame_count: u32, frame_size: UVec2, padding: u32) -> Vec<URect> {
    (0..frame_count)
        .map(|i| {
            let offset_x = i * (frame_size.x + padding);
            let min = UVec2::new(start_min.x + offset_x, start_min.y);
            let max = UVec2::new(min.x + frame_size.x, min.y + frame_size.y);
            URect { min, max }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub fn texture<T, U>(
    layout_size: UVec2,
    start_min: UVec2,
    texture_path: &str,
    texture_entity_type: T,
    frame_count: u32,
    frame_size: UVec2,
    padding: u32,
    make_entity: impl Fn(TextureAtlasLayout, T, String) -> U,
) -> U {
    let frames = generate_frames(start_min, frame_count, frame_size, padding);

    let mut layout = TextureAtlasLayout::new_empty(layout_size);
    for frame in &frames {
        layout.add_texture(*frame);
    }

    make_entity(layout, texture_entity_type, texture_path.to_string())
}