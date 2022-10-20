pub mod vector;
pub use vector::*;

pub mod config;
pub use config::*;

pub mod state;
pub use state::*;

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


/// Implement this trait to access events of the window.
pub trait App {
    /// Gets called right before the main loop.
    fn start(&mut self, _: State) {}

    /// Handle events here.
    /// Make sure to return true if a event was handled, false otherwise.
    fn events(&mut self, _: State, _: Event) -> bool { false }

    /// Gets called once every frame.
    fn update(&mut self, _: State) {}

    /// Create gui stuff here using the context.
    fn gui(&mut self, _: State, _: &gui::Context) {}

    /// Render stuff onto the window here.
    fn render(&self, _: State, _: &mut RenderWindow) {}
}
