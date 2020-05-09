#![allow(dead_code)]
use sfml::system::Vector2f;
use super::vector_math;

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub position: Vector2f,
    pub radius:   f32,
}


impl Circle {
    pub fn intersect(&self, b: &Circle) -> bool {
        vector_math::length_squared(&(self.position - b.position)) < (self.radius + b.radius).powf(2.0)
    }
}