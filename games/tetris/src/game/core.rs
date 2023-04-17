use sfml::{
    window::{Style, Event, Key}, 
    graphics::{RenderWindow, RenderTarget, Color}
};

use super::configuration;

pub fn run() {
    let mut window = RenderWindow::new(
        (480, 640),
        "TETRIS",
        Style::CLOSE,
        &configuration::SETTINGS
    );

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed { 
                    code: Key::Escape,..
                } => return,
                _ => (),
            }
        }

        window.clear(Color::WHITE);
        window.display();
    }
}
