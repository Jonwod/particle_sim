extern crate sfml;
extern crate num;

mod ball;
mod world;
mod geometry;
mod vector_math;
mod interface;
mod slider;

use sfml::window::{ContextSettings, VideoMode, event, window_style};
use sfml::window::event::Event;
use sfml::graphics::{RenderWindow, RenderTarget, Color};
use world::World;
use interface::Interface;


fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                             "SFML Example", window_style::CLOSE,
                                             &ContextSettings::default()).expect("failed to create window");

    let dt = 1. / 60.;
    window.set_framerate_limit(60);

    let mut world = World::new();
    let mut interface = Interface::new();

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::MouseButtonPressed{button, x, y}  => interface.notify_mouse_down(x, y),
                event::MouseButtonReleased{button, x, y} => interface.notify_mouse_up(x, y),
                event::MouseMoved{x, y}                              => interface.notify_mouse_moved(x, y),
                _             => {/* do nothing */}
            }
        }

        // Clear the window
        window.clear(&Color::new_rgb(0, 200, 200));
        world.draw(&mut window);
        interface.draw(&mut window, &world);

        // Display things on screen
        window.display();

        world.update(dt * interface.get_time_factor());
    }
}
