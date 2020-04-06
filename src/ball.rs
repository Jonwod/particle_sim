use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};
use super::geometry::Circle;


#[derive(Debug, Copy, Clone)]
pub struct Ball {
    pub circle:   Circle,
    pub velocity: Vector2f,
    mass: f32,
}


impl Ball {
    pub fn draw(&self, window: &mut RenderWindow) {
        let mut temp_circle = match CircleShape::new() {
            Some(circle) => {
                circle
            },
            None => panic!("Error, cannot create CircleShape")
        };

        temp_circle.set_radius(self.circle.radius);
        temp_circle.set_fill_color(&Color::red());
        temp_circle.set_position(&self.circle.position);
        temp_circle.set_origin(&Vector2f{x: temp_circle.get_radius(), y: temp_circle.get_radius()});
        window.draw(&temp_circle);
    }

    pub fn default() -> Ball {
        Ball{circle: Circle{position: Vector2f{x: 0., y: 0.}, radius: 32.0},
             velocity: Vector2f{x: 0., y: 0.}, mass: 1.0}
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }


    pub fn set_mass(&mut self, m: f32) {
        self.mass = m;
    }


    // Given two balls, presumed to be colliding, returns the post-collision velocities
    // of ball a and ball b respectively
    pub fn resolve_collision(a: & Ball, b: & Ball) -> (Vector2f, Vector2f) {
        let m_sum = a.get_mass() + b.get_mass();
        let va =  a.velocity * (a.get_mass() - b.get_mass()) / (m_sum)
                + b.velocity * (2.0 * b.get_mass()) / (m_sum);
        let vb = a.velocity * (2.0 * a.get_mass()) / (m_sum)
               + b.velocity * (a.get_mass() - b.get_mass()) / (a.get_mass() + b.get_mass());

        (va, vb)
    }
}
