pub fn degrees_to_radians(degrees: f32) -> f32 {
    let pi = std::f32::consts::PI;
    degrees * (pi / 180.0)
}
