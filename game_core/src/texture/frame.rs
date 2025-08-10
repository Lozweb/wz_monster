use bevy::math::{URect, UVec2};

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
