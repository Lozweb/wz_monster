use bevy::prelude::Sprite;

pub fn weapon_rotate_and_flip(
    weapon_sprite: &mut Sprite,
    aim_direction: f32,
) {
    let face_right = is_face_right(radian_to_degrees(aim_direction));
    if face_right {
        weapon_sprite.flip_y = !face_right;
        weapon_sprite.flip_x = face_right;
    } else {
        weapon_sprite.flip_y = !face_right;
        weapon_sprite.flip_x = !face_right;
    }
}

pub fn is_face_right(angle: f32) -> bool {
    (0.0..=90.0).contains(&angle) || (270.0..=360.0).contains(&angle)
}
pub fn radian_to_degrees(radians: f32) -> f32 {
    degrees_normalize(radians.to_degrees())
}
pub fn degrees_normalize(degrees: f32) -> f32 {
    if degrees < 0.0 {
        degrees + 360.0
    } else {
        degrees
    }
}