use crate::{Vector, App};
use sfml::{
    window::{Style, Event},
    system::Vector2,
    graphics::{RenderWindow, RenderTarget, Color}
};


pub enum Config {
    All {size: Vector<u16>, pos: Vector<i32>, style: Style},
    Size {size: Vector<u16>},
    Pos {size: Vector<u16>, pos: Vector<i32>},
    Grid {rows: u16, cols: u16, scale: u16},
}
impl Config {
    pub fn new<Vu16: Into<Vector<u16>>, Vi32: Into<Vector<i32>>>(size: Vu16, pos: Vi32, style: Style) -> Config {
        Config::All { size: size.try_into().unwrap(), pos: pos.try_into().unwrap(), style }
    }
    pub fn size<V: Into<Vector<u16>>>(size: V) -> Config {
        Config::Size { size: size.try_into().unwrap() }
    }
    pub fn pos<Vu16: Into<Vector<u16>>, Vi32: Into<Vector<i32>>>(size: Vu16, pos: Vi32) -> Config {
        Config::Pos { size: size.try_into().unwrap(), pos: pos.try_into().unwrap() }
    }
    pub fn grid(rows: u16, cols: u16, scale: u16) -> Config {
        Config::Grid { rows, cols, scale }
    }
}

pub struct ConfigRes {
    size: Vector<u16>,
    pos: Vector<i32>,
    style: Style
}

impl Config {
    pub fn get(self) -> ConfigRes {
        match self {
            Config::Size { size } => ConfigRes {
                size,
                pos: Vector(150, 150),
                style: Style::CLOSE
            },
            Config::Pos { size, pos } => ConfigRes { 
                size, pos,
                style: Style::CLOSE
            },
            Config::Grid { rows, cols, scale } => ConfigRes { 
                size: Vector(rows * scale, cols * scale),
                pos: Vector(150, 150),
                style: Style::CLOSE
            },
            Config::All { size, pos, style } => ConfigRes { 
                size, pos, style
            },
        }
    }
}


// size: Vector<u16>, pos: Option<Vector<u16>>, 

pub fn run<T: App> (
    title: &str, config: Config, mut app: T
) {
    let config_final = config.get();
    let mut window = RenderWindow::new(
        config_final.size,
        title,
        config_final.style,
        //Style::CLOSE,
        &Default::default(),
    );
    window.set_position(Vector2::new(config_final.pos.0, config_final.pos.1));

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
