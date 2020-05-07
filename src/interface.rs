use sfml::graphics::{RenderWindow, RenderTarget, Text, Font, Transformable};
use super::world::World;
use sfml::system::Vector2f;
use super::slider::Slider;
use super::ball::Ball;

pub struct Interface {
    font: Font,
    time_slider: Slider,
}


impl Interface {
    pub fn vec_to_string(v: &Vector2f) -> String {
        format!("({}, {})", v.x, v.y)
    }


    pub fn new() -> Interface {
        Interface{
            font: Font::new_from_file("data/Ubuntu-R.ttf").expect("failed to load font"),
            time_slider: Slider::new(
                Vector2f{x: 200.0, y: 5.0},
                Vector2f{x: 0.0, y: 90.0},
                Vector2f{x: 20.0, y: 10.0},
                0.5,
                -1.0,
                1.0,
                Font::new_from_file("data/Ubuntu-R.ttf").expect("failed to load font")
            )
        }
    }


    pub fn draw(&self, window: &mut RenderWindow, world: &World) {
        let mut t = Text::new().expect("failed to create text");
        t.set_font(&self.font);
        t.set_string(&format!("v1: {}", Interface::vec_to_string(&world.get_balls()[0].velocity)));
        window.draw(&t);

        t.set_string(&format!("v2: {}", Interface::vec_to_string(&world.get_balls()[1].velocity)));
        t.set_position(&Vector2f{x: 0.0, y: 40.0});
        window.draw(&t);


        self.time_slider.draw(window);

        // let invert_time = self.get_time_factor() < 0.0;

        // let collision_message = match Ball::collision_time(&world.get_balls()[0], &world.get_balls()[1], invert_time) {
        //     Some(dt) => format!("{:.2}", dt),
        //     None => String::from("never")
        // };
        //
        //
        // t.set_string(&format!("time to collision: {}", collision_message));
        // t.set_position(&Vector2f{x: 0.0, y: 400.0});
        // window.draw(&t);
    }


    pub fn get_time_factor(&self) -> f32 {
        self.time_slider.get_value()
    }

    pub fn notify_mouse_down(&mut self, x: i32, y: i32) {
        self.time_slider.notify_mouse_down(x, y)
    }

    pub fn notify_mouse_up(&mut self, x: i32, y: i32) {
        self.time_slider.notify_mouse_up(x, y)
    }

    pub fn notify_mouse_moved(&mut self, x: i32, y: i32) {
        self.time_slider.notify_mouse_moved(x, y)
    }
}
