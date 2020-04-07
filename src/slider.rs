use sfml::graphics::{RenderWindow, RenderTarget, Color, Transformable, Shape, RectangleShape, Font, Text, IntRect};
use sfml::system::{Vector2f, Vector2i};
use num;

pub struct Slider {
    pub size: Vector2f,
    pub position: Vector2f,
    pub handle_size: Vector2f,
    pub handle_position: f32,   // This is normalized between 0 and 1
    pub min: f32,
    pub max: f32,
    pub font: Font,
    grabbed: bool,
}


impl Slider {
    pub fn new(size: Vector2f,
               position: Vector2f,
               handle_size: Vector2f,
               handle_position: f32,   // This is normalized between 0 and 1
               min: f32,
               max: f32,
               font: Font) -> Slider
    {
        Slider{size, position, handle_size, handle_position, min, max, font, grabbed: false}
    }


    pub fn draw(&self, window: &mut RenderWindow) {
        let mut line = RectangleShape::new_init(&Vector2f{x: self.size.x, y: 5.0}).expect("failed to create RectangleShape");
        line.set_position(&self.position);
        line.set_fill_color(&Color::black());
        window.draw(&line);

        let mut handle = RectangleShape::new_init(&self.handle_size).expect("failed to create RectangleShape");
        handle.set_origin(& (self.handle_size / 2.0) );
        handle.set_position(&self.handle_origin());
        handle.set_fill_color(&Color::new_rgb(125, 125, 125));
        window.draw(&handle);

        let mut text = Text::new().expect("failed to create text");
        text.set_font(&self.font);
        text.set_string(&format!("{:.2}", self.get_value()));
        //text.set_position(& (self.position + Vector2f{x: self.size.x + 2.0, y: 0.0}) );
        text.set_position(&self.handle_origin());
        text.set_character_size(12);
        window.draw(&text);
    }

    pub fn handle_origin(& self) -> Vector2f {
        self.position + Vector2f{x: self.handle_position * self.size.x, y: self.size.y / 2.0}
    }

    pub fn get_value(&self) -> f32 {
        self.min + (self.max - self.min) * self.handle_position
    }


    pub fn handle_rect(&self) -> IntRect {
        let extent = self.handle_size / 2.0;
        IntRect{
            left: (self.handle_origin().x - extent.x) as i32,
            top: (self.handle_origin().y - extent.y) as i32,
            width: (self.handle_size.x) as i32,
            height: (self.handle_size.y) as i32,
        }
    }


    pub fn notify_mouse_down(&mut self, x: i32, y: i32) {
        if self.handle_rect().contains(Vector2i{x, y}) {
            self.grabbed = true;
        }
    }

    pub fn notify_mouse_up(&mut self, _: i32, _: i32) {
        self.grabbed = false;
    }

    pub fn notify_mouse_moved(&mut self, x: i32, _: i32) {
        if self.grabbed {
            self.handle_position = num::clamp((x as f32 - self.position.x)/self.size.x, 0.0, 1.0);
        }
    }
}
