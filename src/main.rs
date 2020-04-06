extern crate sfml;

mod ball;
mod world;
mod geometry;
mod vector_math;

use sfml::window::{ContextSettings, VideoMode, event, window_style};
use sfml::graphics::{RenderWindow, RenderTarget, Color};
use world::World;


fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                             "SFML Example", window_style::CLOSE,
                                             &ContextSettings::default()).expect("failed to create window");

    let dt = 1. / 60.;
    window.set_framerate_limit(60);

    let mut world = World::new();

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }

        // Clear the window
        window.clear(&Color::new_rgb(0, 200, 200));
        world.draw(&mut window);

        // Display things on screen
        window.display();

        world.update(dt);
    }
}
