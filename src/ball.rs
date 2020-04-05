use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};


#[derive(Debug, Copy, Clone)]
pub struct Ball {
    pub position: Vector2f,
    pub velocity: Vector2f,
    pub radius: f32
}


impl Ball {
    pub fn draw(&self, window: &mut RenderWindow) {
        let mut temp_circle = match CircleShape::new() {
            Some(circle) => {
                circle
            },
            None => panic!("Error, cannot create CircleShape")
        };

        temp_circle.set_radius(self.radius);
        temp_circle.set_fill_color(&Color::red());
        temp_circle.set_position(&self.position);
        temp_circle.set_origin(&Vector2f{x: temp_circle.get_radius(), y: temp_circle.get_radius()});
        window.draw(&temp_circle);
    }

    pub fn default() -> Ball {
        Ball{position: Vector2f{x: 0., y: 0.}, velocity: Vector2f{x: 0., y: 0.}, radius: 32.}
    }
}
