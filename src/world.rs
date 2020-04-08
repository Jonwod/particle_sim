use super::ball::Ball;
use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RectangleShape, Transformable, Shape, Color, RenderTarget};
use crate::geometry::Circle;

static G: f32 = 0.0;


pub struct World {
    balls: [Ball; 2],
    floor_level: f32,
}


impl World {
    pub fn new() -> World {
        let mut world = World{ balls: [Ball::default(); 2], floor_level: 500.0};
        world.balls[0].circle.position.x = 100.0;
        world.balls[0].velocity.x = 100.0;
        world.balls[1].circle.position.x = 600.0;
        world.balls[1].velocity.x = -200.0;
        world.balls[1].set_mass(1.0);
        for ball in &mut world.balls {
            ball.circle.position.y = 200.0;
        }
        world
    }


    pub fn update(&mut self, dt: f32) {
        for ball in &mut self.balls {
            ball.velocity.y += G * dt;
        }

        let mut projected_circles: Vec<Circle> = self.balls.iter().map(|b| Circle{position: b.circle.position + b.velocity*dt, radius: b.circle.radius}).collect();

        for i in 0..projected_circles.len() {
            for j in (i+1)..projected_circles.len() {
                if projected_circles[i].intersect(&projected_circles[j]) {
                    let (vi, vj) = Ball::resolve_collision(& self.balls[i], & self.balls[j]);
                    self.balls[i].velocity = vi;
                    self.balls[j].velocity = vj;
                    projected_circles[i].position = self.balls[i].circle.position + self.balls[i].velocity * dt;
                    projected_circles[j].position = self.balls[j].circle.position + self.balls[j].velocity * dt
                }
            }
        }

        for i in 0..projected_circles.len() {
            self.balls[i].circle.position = projected_circles[i].position;
        }


        //for ball in &mut self.balls {
            //ball.velocity.y += G * dt;
            // let projected_position = ball.circle.position + ball.velocity * dt;
            // for other_ball in &self.balls {
            //     if other_ball as *const int != ball as *const int {
            //         let projected_circle = Circle{position: projected_position, radius: ball.circle.radius};
            //         if projected_circle.intersect(&other_ball.circle) {
            //
            //         }
            //     }
            // }

            // if projected_position.y + ball.circle.radius > self.floor_level {
            //     let pre_impact_ds = self.floor_level - (projected_position.y + ball.circle.radius);
            //     let pre_impact_dt = pre_impact_ds / ball.velocity.y;
            //     let post_impact_dt = dt - pre_impact_dt;
            //     ball.velocity.y = -ball.velocity.y;     // Perfectly elastic collision
            //     ball.circle.position.y += pre_impact_ds + ball.velocity.y * post_impact_dt;
            //     ball.circle.position.x += ball.velocity.x * dt;
            // }
            // else {
            //     ball.circle.position = projected_position;
            // }
        //}
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
