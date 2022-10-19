use crate::{Vector, App};
use sfml::{
    window::{Style, Event},
    system::Vector2,
    graphics::{RenderWindow, RenderTarget, Color}
};


pub enum WindowConfig {
    Size {size: Vector<u16>},
    SizePos {size: Vector<u16>, pos: Vector<i32>},
    Grid {rows: u16, cols: u16, scale: u16},
    All {size: Vector<u16>, pos: Vector<i32>, style: Style}
}

pub struct WindowConfigRes {
    size: Vector<u16>,
    pos: Vector<i32>,
    style: Style
}

impl WindowConfig {
    pub fn get(self) -> WindowConfigRes {
        match self {
            WindowConfig::Size { size } => WindowConfigRes {
                size,
                pos: Vector(150, 150),
                style: Style::CLOSE
            },
            WindowConfig::SizePos { size, pos } => WindowConfigRes { 
                size, pos,
                style: Style::CLOSE
            },
            WindowConfig::Grid { rows, cols, scale } => WindowConfigRes { 
                size: Vector(rows * scale, cols * scale),
                pos: Vector(150, 150),
                style: Style::CLOSE
            },
            WindowConfig::All { size, pos, style } => WindowConfigRes { 
                size, pos, style
            },
        }
    }
}


// size: Vector<u16>, pos: Option<Vector<u16>>, 

pub fn run<T: App> (
    title: &str, config: WindowConfig, mut app: T
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
