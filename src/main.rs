use sfml::{
    graphics::{Color, RectangleShape, CustomShapePoints, RenderTarget, RenderWindow, Shape, Transformable},
    system::{Vector2f, Vector2},
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

    let mut shape = RectangleShape::new();
    shape.set_size(Vector2f{x:10., y:20.});
    shape.set_position(Vector2f{x:20., y:20.});
    shape.set_fill_color(Color::RED);
    shape.set_outline_color(Color::GREEN);
    shape.set_outline_thickness(3.);

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
        window.draw(&shape);
        window.display();
    }
}
