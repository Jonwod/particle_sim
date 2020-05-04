use sfml::system::Vector2f;


pub fn length_squared(vec: &Vector2f) -> f32 {
    vec.x * vec.x + vec.y * vec.y
}


pub fn angle_rad(vec: &Vector2f) -> f32 {
    f32::atan2(vec.y, vec.x)
}


pub fn rotate(vec: &Vector2f, angle_rad: f32) -> Vector2f {
    let s = f32::sin(angle_rad);
    let c = f32::cos(angle_rad);
    Vector2f{x: vec.x * c - vec.y *s, y: vec.x*s + vec.y * c}
}


pub fn dot_product(a: &Vector2f, b: &Vector2f) -> f32 {
    a.x * b.x  + a.y * b.y
}
