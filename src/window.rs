use crate::{Vector, App, Style, Event, RenderWindow, Color};
use sfml::{
    system::Vector2,
    graphics::RenderTarget
};


/// This struct stores the necessary window attributes.
pub struct Config {
    size: Vector<u16>,
    pos: Vector<i32>,
    style: Style
}
impl Config {
    /// Create a config object based on all options.
    /// 
    /// ### Arguments
    /// * `size`: [Into]<[Vector]<[u16]>> - The window size.
    /// * `pos`: [Into]<[Vector]<[u16]>> - The window pos.
    /// * `style`: Style - Additional sfml window style options.
    pub fn new<Vu16: Into<Vector<u16>>, Vi32: Into<Vector<i32>>>(size: Vu16, pos: Vi32, style: Style) -> Config {
        Config { size: size.try_into().unwrap(), pos: pos.try_into().unwrap(), style }
    }

    /// Create a config object based on the size.
    /// All other attributes will be set to default.
    /// 
    /// ### Arguments
    /// * `size`: [Into]<[Vector]<[u16]>> - The window size.
    pub fn size<V: Into<Vector<u16>>>(size: V) -> Config {
        Config { size: size.try_into().unwrap(), pos: Vector(150, 150), style: Style::CLOSE }
    }

    /// Create a config object based on the size & position.
    /// All other attributes will be set to default.
    /// 
    /// ### Arguments
    /// * `size`: [Into]<[Vector]<[u16]>> - The window size.
    /// * `pos`: [Into]<[Vector]<[u16]>> - The window pos.
    pub fn pos<Vu16: Into<Vector<u16>>, Vi32: Into<Vector<i32>>>(size: Vu16, pos: Vi32) -> Config {
        Config { size: size.try_into().unwrap(), pos: pos.try_into().unwrap(), style: Style::CLOSE }
    }

    /// Create a config object.
    /// The window size will be calculated automatically based on the grid size & scale.
    pub fn grid(rows: u16, cols: u16, scale: u16) -> Config {
        Config { size: Vector(cols * scale, rows * scale), pos: Vector(150, 150), style: Style::CLOSE }
    }
}


pub fn run<T: App> (
    title: &str, config: Config, mut app: T
) {
    let mut window = RenderWindow::new(
        config.size,
        title,
        config.style,
        //Style::CLOSE,
        &Default::default(),
    );
    window.set_position(Vector2::new(config.pos.0, config.pos.1));

    while window.is_open() {

        // Events
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => { if app.events(event) {break;} }
            }
        }

        app.update(0.);

        window.clear(Color::BLACK);
        app.render(&mut window);
        window.display();
    }
}
