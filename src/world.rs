use super::ball::Ball;
use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RectangleShape, Transformable, Shape, Color, RenderTarget};

static G: f32 = 0.0;


pub struct World {
    balls: [Ball; 2],
    floor_level: f32,
}


impl World {
    pub fn new() -> World {
        let mut world = World{ balls: [Ball::default(); 2], floor_level: 500.0};
        world.balls[0].circle.position.x = 200.0;
        world.balls[0].circle.position.y = 100.0;
        world.balls[0].velocity.x = 100.0;
        world.balls[0].velocity.y = 50.0;
        world.balls[1].circle.position.x = 400.0;
        world.balls[1].circle.position.y = 200.0;
        world.balls[1].velocity.x = -30.0;
        world.balls[1].set_mass(1.0);
        world
    }


    pub fn update(&mut self, mut dt: f32) {
        for ball in &mut self.balls {
            ball.velocity.y += G * dt;
        }

        match Ball::collision_time(&self.balls[0], &self.balls[1], dt < 0.0) {
            Some(dt1) => {
                if dt1.abs() < dt.abs() {
                    self.balls[0].circle.position = self.balls[0].circle.position + self.balls[0].velocity * dt1;
                    self.balls[1].circle.position = self.balls[1].circle.position + self.balls[1].velocity * dt1;
                    let (v0, v1) = Ball::resolve_collision(&self.balls[0], &self.balls[1]);
                    self.balls[0].velocity = v0;
                    self.balls[1].velocity = v1;
                    dt -= dt1;
                }
            },
            None => { }
        }

        self.balls[0].circle.position = self.balls[0].circle.position + self.balls[0].velocity * dt;
        self.balls[1].circle.position = self.balls[1].circle.position + self.balls[1].velocity * dt;
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


    pub fn get_balls(&self) -> &[Ball; 2] {
        &self.balls
    }
}
