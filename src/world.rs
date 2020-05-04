use super::ball::Ball;
use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RectangleShape, Transformable, Shape, Color, RenderTarget, FloatRect};
use super::plane::Plane;

const G: f32 = 0.0;

const NUM_BALLS: usize = 2;


pub struct World {
    balls: [Ball; NUM_BALLS],
    walls: FloatRect,
}


#[derive(Copy, Clone, Debug)]
enum CollisionKind {
    Wall,
    Ball,
}


#[derive(Copy, Clone, Debug)]
struct Collision {
    kind: CollisionKind,
    time: f32,
    collider_index: usize,
}


fn replace_if_sooner(old_collision: &mut Option<Collision>, new_collision: & Collision) {
    if let Some(old_col) = old_collision {
        if new_collision.time.abs() < old_col.time.abs() {
            *old_collision = Some(*new_collision);
        }
    } else {
        *old_collision = Some(*new_collision);
    }
}


// Returns the index of the soonest collision. Returns the first index
// if there are no valid collisions
fn soonest_collision(collisions: &[Option<Collision>]) -> usize {
    let mut soonest_index = 0;
    let mut soonest_time = if let Some(col) = collisions[0] {
        col.time
    } else {
        std::f32::INFINITY
    };

    for i in 1..collisions.len() {
        if let Some(col) = collisions[i] {
            if col.time.abs() < soonest_time {
                soonest_index = i;
                soonest_time = col.time.abs();
            }
        }
    }

    soonest_index
}


impl World {
    pub fn new() -> World {
        let mut world = World{ balls: [Ball::default(); NUM_BALLS],
            walls: FloatRect{left: 10.0, top: 150.0, width: 800.0,  height: 800.0}};

        world.balls[0].circle.position.x = 200.0;
        world.balls[0].circle.position.y = 300.0;
        world.balls[0].velocity.x = 2300.0;
        world.balls[0].velocity.y = 500.0;
        world.balls[1].circle.position.x = 400.0;
        world.balls[1].circle.position.y = 400.0;
        world.balls[1].velocity.x = -3000.0;
        world.balls[1].set_mass(1.0);
        //
        // world.balls[2].set_position(500.0, 500.0);
        // world.balls[3].set_position(600.0, 600.0);

        world
    }



    pub fn update(&mut self, mut dt: f32) {
        for ball in &mut self.balls {
            ball.velocity.y += G * dt;
        }

        let walls = self.get_wall_planes();

        while dt != 0.0 {
            let mut collisions: [Option<Collision>; NUM_BALLS] = [None; NUM_BALLS];

            for i in 0..self.balls.len() {
                for j in (i + 1)..self.balls.len() {
                    if let Some(t) = Ball::collision_time(&self.balls[i], &self.balls[j], dt < 0.0) {
                        if t != 0.0  &&  t.abs() <= dt.abs() {
                            replace_if_sooner(&mut collisions[i], &Collision { kind: CollisionKind::Ball, time: t, collider_index: j});
                            replace_if_sooner(&mut collisions[j], &Collision { kind: CollisionKind::Ball, time: t, collider_index: i});
                        }
                    }
                }

                for j in 0..walls.len()  {
                    if let Some(t) = self.balls[i].plane_collision_time(&walls[j], dt < 0.0) {
                        if t != 0.0  &&  t.abs() <= dt.abs() {
                            replace_if_sooner(&mut collisions[i], &Collision { kind: CollisionKind::Wall, time: t, collider_index: j});
                        }
                    }
                }
            }

            let soonest_collision_index = soonest_collision(&collisions);
            if let Some(soonest_col) = collisions[soonest_collision_index] {
                let a = soonest_collision_index;
                let b = soonest_col.collider_index;
                match soonest_col.kind {
                    CollisionKind::Ball => {
                        self.balls[a].displace(&(self.balls[a].velocity * soonest_col.time));
                        self.balls[b].displace(&(self.balls[b].velocity * soonest_col.time));
                        let (va, vb) = Ball::resolve_collision(&self.balls[a], &self.balls[b]);
                        self.balls[a].velocity = va;
                        self.balls[b].velocity = vb;
                    },
                    CollisionKind::Wall => {
                        self.balls[a].displace(&(self.balls[a].velocity * soonest_col.time));
                        self.balls[a].resolve_plane_collision(&walls[b]);
                    }
                }
                dt -= soonest_col.time;
            } else {
                // This means there are no further collisions to resolve for the remaining dt
                break;
            }
        }

        for ball in &mut self.balls {
            ball.displace(&(ball.velocity * dt));
        }
    }



    pub fn draw(&self, window: &mut RenderWindow) {
        for ball in &self.balls {
            ball.draw(window);
        }
        self.draw_walls(window);
    }



    fn draw_walls(&self, window: &mut RenderWindow) {
        let thickness = 10.0;
        let mut rect = RectangleShape::new_init(&Vector2f{ x: self.walls.width, y: thickness}).expect("failed to create RectangleShape");
        rect.set_fill_color(&Color::new_rgb(125, 125, 125));

        rect.set_position2f(self.walls.left, self.walls.top + self.walls.height);
        window.draw(&rect);

        rect.set_position(&Vector2f{x: self.walls.left, y: self.walls.top - thickness});
        window.draw(&rect);

        rect.set_size2f(thickness, self.walls.height);

        rect.set_position2f(self.walls.left + self.walls.width, self.walls.top);
        window.draw(&rect);

        rect.set_position2f(self.walls.left - thickness, self.walls.top);
        window.draw(&rect);
    }



    pub fn get_balls(&self) -> &[Ball; NUM_BALLS] {
        &self.balls
    }



    fn get_wall_planes(&self) -> [Plane; 4] {
        [
            Plane{position: Vector2f{x: self.walls.left, y: 0.0}, normal: Vector2f{x: 1.0, y: 0.0}},
            Plane{position: Vector2f{x: self.walls.left + self.walls.width, y: 0.0}, normal: Vector2f{x: -1.0, y: 0.0}},
            Plane{position: Vector2f{x: 0.0, y: self.walls.top}, normal: Vector2f{x: 0.0, y: 1.0}},
            Plane{position: Vector2f{x: 0.0, y: self.walls.top + self.walls.height}, normal: Vector2f{x: 0.0, y: -1.0}}
        ]
    }
}
