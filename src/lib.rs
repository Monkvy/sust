pub mod vector;
pub use vector::*;

pub mod window;
pub use window::*;

pub mod shape;
pub use shape::*;

pub mod grid;
pub use grid::*;

pub use sfml::{
    window::{Event, mouse, Style, Key},
    graphics::{RenderWindow, Color},
};
pub use egui_sfml::egui as gui;
use std::sync::Mutex;


/// Stores the current mouse button & position.
/// If button == None, the pos == to the mouse position
/// of the last mouse press / release.
#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    pub button: Option<mouse::Button>,
    pub pos: Vector<u16>
}
pub static MOUSE_STATE: Mutex<MouseState> = Mutex::new(MouseState{button: None, pos: Vector(0, 0)});

/// Implement this trait to access, change & render data
/// while inside the main loop of the window.
pub trait App {
    fn events(&mut self, event: Event) -> bool;
    fn update(&mut self, dt: f32);
    fn gui(&mut self, ctx: &gui::Context);
    fn render(&self, window: &mut RenderWindow);
}
