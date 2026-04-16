pub fn is_close(left: f32, right: f32) -> bool {
    let diff = (left - right).abs();
    diff <= f32::EPSILON
}
