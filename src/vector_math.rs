use sfml::system::Vector2f;


pub fn length_squared(vec: &Vector2f) -> f32 {
    vec.x * vec.x + vec.y * vec.y
}