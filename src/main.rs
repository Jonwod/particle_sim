extern crate sfml;
extern crate num;
extern crate rand;

mod ball;
mod world;
mod geometry;
mod vector_math;
mod interface;
mod slider;
mod math;
mod plane;

use sfml::window::{ContextSettings, Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, Color};
use world::World;
use interface::Interface;


fn main() {
        // Create the window of the application
    let mut window = RenderWindow::new((1000, 1000),
                                             "SFML Example", Style::CLOSE,
                                             &ContextSettings::default());

    let dt = 1. / 60.;
    window.set_framerate_limit(60);

    let mut world = World::new();
    let mut interface = Interface::new();

    while window.is_open() {
        // Handle events
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed{button: _, x, y}  => interface.notify_mouse_down(x, y),
                Event::MouseButtonReleased{button: _, x, y} => interface.notify_mouse_up(x, y),
                Event::MouseMoved{x, y}                     => interface.notify_mouse_moved(x, y),
                _             => {/* do nothing */}
            }
        }

        // Clear the window
        window.clear(&Color::rgb(0, 200, 200));
        world.draw(&mut window);
        interface.draw(&mut window, &world);

        // Display things on screen
        window.display();

        world.update(dt * interface.get_time_factor());
    }
}
