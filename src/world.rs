use super::ball::Ball;
use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RectangleShape, Transformable, Shape, Color, RenderTarget, FloatRect};
use super::plane::Plane;
use rand::Rng;
use super::vector_math;




pub struct World {
    balls: Vec<Ball>,
    walls: [Plane; 4],
}


#[derive(Copy, Clone, Debug, PartialEq)]
enum CollisionKind {
    Wall,
    Ball,
}


#[derive(Copy, Clone, Debug)]
struct Collision {
    kind: CollisionKind,
    time: f32,
    ball_index: usize,
    collider_index: usize,
}


impl World {
    pub fn new() -> World {
        let walls_rect = FloatRect{left: 10.0, top: 150.0, width: 800.0,  height: 800.0};

        let mut world = World{ balls: vec![Ball::default(); 120],
            walls: World::rect_to_planes(&walls_rect)};

        let offset = Ball::default().circle.radius * 3.0;
        let origin = Vector2f{x: walls_rect.left + offset, y: walls_rect.top + offset};
        let sides = (world.balls.len() as f32).sqrt().ceil() as usize;
        let spacing = (walls_rect.width - 2.0 * offset) / sides as f32;
        for i in 0..world.balls.len() {
            let x = (i % sides) as f32 * spacing;
            let y = (i / sides) as f32 * spacing;
            world.balls[i].circle.position = Vector2f{x, y} + origin;
        }

        let mut rng = rand::thread_rng();
        let max_vel = 2000.0;
        for ball in &mut world.balls {
            let intensity = rng.gen_range(0.0, max_vel);
            let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
            ball.velocity = vector_math::rotate(&Vector2f{x: intensity, y: 0.0}, angle);
        }

        world
    }



    pub fn update(&mut self, mut dt: f32) {
        // A fully robust collision policy requires finding the soonest collision(s), advancing time to
        // the point of collision(s), resolving the collisions and then repeating the process with re-computed velocities

        while dt != 0.0 {
            let soonest_collisions = self.get_soonest_collisions(dt < 0.0);
            if soonest_collisions.len() > 0 {
                if soonest_collisions[0].time.abs() > dt.abs() {
                    break;  // No more collisions this frame
                }
                dt -= self.resolve_collisions(&soonest_collisions);
            }
            else {
                break;
            }
        }

        if dt != 0.0 {
            for ball in &mut self.balls {
                ball.displace(&(ball.velocity * dt));
            }
        }
    }



    // Resolves the specified collisions, which should be concurrent. Advances simulation to
    // the time of collision. Returns the elapsed time.
    fn resolve_collisions(&mut self, collisions: &Vec<Collision>) -> f32 {
        if collisions.len() == 0 { return 0.0; }

        let t = collisions[0].time;

        // Advance the simulation to the point of the collision
        for ball in &mut self.balls {
            ball.displace(&(ball.velocity * t))
        }

        for collision in collisions {
            self.resolve_collision(collision);
        }

        t
    }


    fn resolve_collision(&mut self, collision: &Collision) {
        let a = collision.ball_index;
        let b = collision.collider_index;
        match collision.kind {
            CollisionKind::Ball => {
                let (va, vb) = Ball::resolve_collision(&self.balls[a], &self.balls[b]);
                self.balls[a].velocity = va;
                self.balls[b].velocity = vb;
            },
            CollisionKind::Wall => {
                self.balls[a].resolve_plane_collision(&self.walls[b]);
            }
        };
    }


    // Excludes collisions at time 0 as these should have been resolved in the previous iteration
    fn get_soonest_collisions(&self, invert_time: bool) -> Vec<Collision> {
        let mut soonest_collisions: Vec<Collision> = Vec::new();

        let mut add_collision_perhaps = |new_collision: &Collision| {
            if new_collision.time == 0.0 { return; }

            if soonest_collisions.len() > 0 {
                if new_collision.time.abs() < soonest_collisions[0].time.abs() {
                    soonest_collisions.clear();
                    soonest_collisions.push(*new_collision);
                }
                else if new_collision.time == soonest_collisions[0].time {
                    soonest_collisions.push(*new_collision);
                }
            }
            else {
                soonest_collisions.push(*new_collision);
            }
        };


        for i in 0..self.balls.len() {
            for j in (i + 1)..self.balls.len() {
                if let Some(t) = Ball::collision_time(&self.balls[i], &self.balls[j], invert_time) {
                    add_collision_perhaps(&Collision{kind: CollisionKind::Ball, time: t, ball_index: i, collider_index: j});
                }
            }

            for j in 0..self.walls.len()  {
                if let Some(t) = self.balls[i].plane_collision_time(&self.walls[j], invert_time) {
                    add_collision_perhaps(&Collision{kind: CollisionKind::Wall, time: t, ball_index: i, collider_index: j});
                }
            }
        }

        if soonest_collisions.len() > 1 {
            println!("soonest_collisions.len(): {}", soonest_collisions.len());
        }

        soonest_collisions
    }



    pub fn draw(&self, window: &mut RenderWindow) {
        for ball in &self.balls {
            ball.draw(window);
        }
        self.draw_walls(window);
    }



    fn draw_walls(&self, window: &mut RenderWindow) {
        let thickness = 10.0;
        let rect = self.bounding_rect();
        let mut shape = RectangleShape::with_size(Vector2f{ x: rect.width, y: thickness});
        shape.set_fill_color(&Color::rgb(125, 125, 125));

        shape.set_position(Vector2f{x: rect.left, y: rect.top + rect.height});
        window.draw(&shape);

        shape.set_position(Vector2f{x: rect.left, y: rect.top - thickness});
        window.draw(&shape);

        shape.set_size(Vector2f{x: thickness, y: rect.height});

        shape.set_position(Vector2f{x: rect.left + rect.width, y: rect.top});
        window.draw(&shape);

        shape.set_position(Vector2f{x: rect.left - thickness, y: rect.top});
        window.draw(&shape);
    }


    pub fn get_balls(&self) -> &Vec<Ball>{
        &self.balls
    }


    // The rectangle representing the box in which the particles are contained
    fn bounding_rect(&self) -> FloatRect {
        let left = self.walls[0].position.x;
        let top = self.walls[2].position.y;
        let width = self.walls[1].position.x - left;
        let height = self.walls[3].position.y - top;
        FloatRect{left, top, width, height}
    }


    fn rect_to_planes(rect: &FloatRect) -> [Plane; 4] {
        [
            Plane{position: Vector2f{x: rect.left, y: 0.0}, normal: Vector2f{x: 1.0, y: 0.0}},
            Plane{position: Vector2f{x: rect.left + rect.width, y: 0.0}, normal: Vector2f{x: -1.0, y: 0.0}},
            Plane{position: Vector2f{x: 0.0, y: rect.top}, normal: Vector2f{x: 0.0, y: 1.0}},
            Plane{position: Vector2f{x: 0.0, y: rect.top + rect.height}, normal: Vector2f{x: 0.0, y: -1.0}}
        ]
    }


}
