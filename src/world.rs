use super::ball::Ball;
use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RectangleShape, Transformable, Shape, Color, RenderTarget};


static G: f32 = 98.0;


pub struct World {
    balls: [Ball; 2],
    floor_level: f32,
}


impl World {
    pub fn new() -> World {
        let mut world = World{ balls: [Ball::default(); 2], floor_level: 500.0};
        world.balls[0].position.x = 100.0;
        world.balls[1].position.x = 600.0;
        for ball in &mut world.balls {
            ball.position.y = 200.0;
        }
        world
    }


    pub fn update(&mut self, dt: f32) {
        for ball in &mut self.balls {
            ball.velocity.y += G * dt;
            let projected_position = ball.position + ball.velocity * dt;
            if projected_position.y + ball.radius > self.floor_level {
                let pre_impact_ds = self.floor_level - (projected_position.y + ball.radius);
                let pre_impact_dt = pre_impact_ds / ball.velocity.y;
                let post_impact_dt = dt - pre_impact_dt;
                ball.velocity.y = -ball.velocity.y;     // Perfectly elastic collision
                ball.position.y += pre_impact_ds + ball.velocity.y * post_impact_dt;
                ball.position.x += ball.velocity.x * dt;
            } else {
                ball.position = projected_position;
            }
        }
    }


    pub fn draw(&self, window: &mut RenderWindow) {
        for ball in &self.balls {
            ball.draw(window);
        }
        self.draw_floor(window);
    }


    fn draw_floor(&self, window: &mut RenderWindow) {
        let mut rect = RectangleShape::new_init(&Vector2f{ x: 2000.0, y: 2000.0}).expect("failed to create RectangleShape");
        rect.set_position(&Vector2f{x: -1000.0, y: self.floor_level});
        rect.set_fill_color(&Color::new_rgb(100, 100, 100));
        window.draw(&rect);
    }
}
