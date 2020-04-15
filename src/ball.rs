use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};
use super::geometry::Circle;
use super::math;
use crate::vector_math::{angle_rad, rotate};


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

    pub fn get_position(&self) -> Vector2f {
        self.circle.position
    }


    // Given two balls, presumed to be colliding, returns the post-collision velocities
    // of ball a and ball b respectively
    pub fn resolve_collision(a: & Ball, b: & Ball) -> (Vector2f, Vector2f) {
        // We solve the problem in 2D by simply finding the axis of collision
        // and then just solving the problem as a 1D collision along that axis.
        // The velocity components perpendicular to the axis are unaffected.
        let collision_axis = a.get_position() - b.get_position();
        let axis_angle = angle_rad(&collision_axis);
        let ua_loc = rotate(&a.velocity, -axis_angle);
        let ub_loc = rotate(&b.velocity, -axis_angle);

        let m_sum = a.get_mass() + b.get_mass();
        let va_loc =  ua_loc.x * (a.get_mass() - b.get_mass()) / (m_sum)
                          + ub_loc.x * (2.0 * b.get_mass()) / (m_sum);
        let vb_loc = ua_loc.x * (2.0 * a.get_mass()) / (m_sum)
                        + ub_loc.x * (a.get_mass() - b.get_mass()) / (a.get_mass() + b.get_mass());

        let va = rotate(&Vector2f{x: va_loc, y: ua_loc.y} , axis_angle);
        let vb = rotate(&Vector2f{x: vb_loc, y: ub_loc.y}, axis_angle);
        (va, vb)
    }


    // Given two balls, returns the time until they will collide
    // Currently only works on the x axis
    pub fn collision_time(ball1: & Ball, ball2: & Ball, invert_time: bool) -> Option<f32> {
        // We approach this by finding the roots of a quadratic function of dt
        let v1 = ball1.velocity.x;
        let v2 = ball2.velocity.x;
        let x1 = ball1.circle.position.x;
        let x2 = ball2.circle.position.x;
        let r1 = ball1.circle.radius;
        let r2 = ball2.circle.radius;
        let a = v1 * v1 - 2.0 * v1 * v2 + v2 * v2;
        let b = 2.0 * x1 * v1 - 2.0*x1*v2 - 2.0*x2*v1 + 2.0*x2*v2;
        let c = x1*x1 - 2.0*x1*x2 + x2*x2 - (r1 + r2).powf(2.0);
        match math::find_roots(a, b, c) {
            Some((dt1, dt2)) => {
                // If both are positive then will be the smallest one,
                // as the larger will represent the balls touching but
                // on the other side.
                let (min, max) = if dt1 < dt2 {(dt1, dt2)} else {(dt2, dt1)};
                if invert_time {
                    if max <= 0.0 {
                        Some(max)
                    } else if min <= 0.0 {
                        Some(min)
                    }
                    else {
                        None
                    }
                }
                else {
                    if min >= 0.0 {
                        Some(min)
                    } else if max >= 0.0 {
                        Some(max)
                    } else {
                        None
                    }
                }
            },
            None => None,
        }
    }
}
