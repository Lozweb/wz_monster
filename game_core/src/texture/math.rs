pub fn is_face_right(aim_direction: f32) -> bool {
    let angle = radian_to_degrees(aim_direction);
    (0.0..=90.0).contains(&angle) || (270.0..=360.0).contains(&angle)
}
fn radian_to_degrees(radians: f32) -> f32 {
    degrees_normalize(radians.to_degrees())
}
fn degrees_normalize(degrees: f32) -> f32 {
    if degrees < 0.0 {
        degrees + 360.0
    } else {
        degrees
    }
}

