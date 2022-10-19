pub mod vector;
pub use vector::*;

pub mod window;
pub use window::*;

pub mod shape;
pub use shape::*;

pub mod grid;
pub use grid::*;

pub use sfml::{
    window::{Event, mouse},
    graphics::{RenderWindow, Color}
};


pub trait App {
    fn events(&mut self, event: Event) -> bool;
    fn update(&mut self, dt: f32);
    fn render(&self, window: &mut RenderWindow);
}
