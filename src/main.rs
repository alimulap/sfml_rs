use sfml::{
    window::{ContextSettings, Style, Event, Key}, 
    graphics::{RenderWindow, RenderTarget, Color}
};

fn main() {
    let settings = ContextSettings {
        antialiasing_level: 2,
        ..Default::default()
    };

    let mut window = RenderWindow::new(
        (480, 640),
        "SFML",
        Style::CLOSE,
        &settings,
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
