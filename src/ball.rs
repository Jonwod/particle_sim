use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};
use super::geometry::Circle;
use super::math;
use crate::vector_math::{angle_rad, rotate, dot_product};
use super::plane::Plane;


#[derive(Debug, Copy, Clone)]
pub struct Ball {
    pub circle:   Circle,
    pub velocity: Vector2f,
    mass: f32,
}


impl Ball {
    pub fn draw(&self, window: &mut RenderWindow) {
        let mut temp_circle = CircleShape::new().expect("Error, failed to create CircleShape");

        temp_circle.set_radius(self.circle.radius);
        temp_circle.set_fill_color(&Color::red());
        temp_circle.set_position(&self.circle.position);
        temp_circle.set_origin(&Vector2f{x: temp_circle.get_radius(), y: temp_circle.get_radius()});
        window.draw(&temp_circle);
    }

    pub fn default() -> Ball {
        Ball{circle: Circle{position: Vector2f{x: 0., y: 0.}, radius: 16.0},
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

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.circle.position.x = x;
        self.circle.position.y = y;
    }


    pub fn displace(&mut self, offset: &Vector2f) {
        self.circle.position = self.circle.position + *offset;
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


    // Given two balls, returns the time until they will collide or None if they are not going
    // to collide in future (in the past if invert_time is set true)
    pub fn collision_time(ball1: &Ball, ball2: &Ball, invert_time: bool) -> Option<f32> {
        // We approach this by finding the roots of a quadratic function of dt
        let u1 = ball1.velocity;
        let u2 = ball2.velocity;
        let i1 = ball1.get_position();  // initial position
        let i2 = ball2.get_position();  // initial position
        let r1 = ball1.circle.radius;
        let r2 = ball2.circle.radius;

        // The coefficients for the quadratic formula:
        let a = (u1.x - u2.x).powi(2) + (u1.y - u2.y).powi(2);
        let b = 2.0 * ( (i1.x - i2.x)*(u1.x - u2.x) + (i1.y - i2.y)*(u1.y - u2.y) );
        let c = -2.0 * (i1.x*i2.x + i1.y*i2.y) + i1.x.powi(2) + i2.x.powi(2) +
                    i1.y.powi(2) + i2.y.powi(2) - (r1+r2).powi(2);

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
            None => None
        }
    }


    pub fn resolve_plane_collision(&mut self, plane: &Plane) {
        // v projected onto the plane normal
        let v = dot_product(&plane.normal, &self.velocity);
        self.velocity = self.velocity - plane.normal * 2.0 * v;
    }


    pub fn plane_collision_time(&self, plane: &Plane, invert_time: bool) -> Option<f32> {
        let v = dot_product(&plane.normal, &self.velocity);
        if v == 0.0 {
            return None;
        }

        let initial_pos = dot_product(&plane.normal, &(self.get_position() - plane.position));
        // the position, in plane-space, at which the ball is touching the plane
        let final_pos = self.circle.radius;
        // The displacement from the ball's current position at which it hits the plane
        let s = final_pos - initial_pos;

        let t = s / v;

        if t >= 0.0 {
            if invert_time {None} else {Some(t)}
        }
        else {
            if invert_time {Some(t)} else {None}
        }
    }
}
