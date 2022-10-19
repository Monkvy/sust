use sfml::{
    graphics::{Color, RenderWindow, RenderTarget},
    window::{Event, Key, Style},
};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Sust window",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);


    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.display();
    }
}
