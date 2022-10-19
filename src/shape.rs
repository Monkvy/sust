use crate::{Vector, Color};
use sfml::{
    graphics::{RenderWindow, RenderTarget, RectangleShape, Transformable, Shape, CircleShape},
    system::Vector2f
};


/// This struct stores a rectangle that
/// can be rendered to the window.
pub struct Rect {
    pos: Vector<u16>,
    size: Vector<u16>,
    color: Color,
    border_size: Option<u16>,
    border_color: Option<Color>,
}
impl Rect {
    /// Create a new rectangle that can be rendered to the window.
    /// 
    /// ### Arguments
    /// * `pos`: [Into]<[Vector]<[u16]>> - The topleft position.
    /// * `size`: [Into]<[Vector]<[u16]>> - The width & height.
    /// * `color`: [Color] - The fill color.
    pub fn new<V: Into<Vector<u16>>>(pos: V, size: V, color: Color) -> Rect {
        Rect { pos: pos.try_into().unwrap(), size: size.try_into().unwrap(), color, border_size: None, border_color: None }
    }

    /// Creates a border around the rect.
    pub fn border(&mut self, border_size: u16, border_color: Color) {
        self.border_size = Some(border_size);
        self.border_color = Some(border_color);
    }

    /// Changed the center rect position
    pub fn center(&mut self, center: Vector<u16>) {
        self.pos = center - self.size / 2;
    }

    /// Renders the rect to the given window.
    pub fn render(&self, window: &mut RenderWindow) {
        let mut r = RectangleShape::new();
        let pos = self.pos.cast::<f32>();
        let size = self.size.cast::<f32>();

        r.set_position(Vector2f::new(pos.0, pos.1));
        r.set_size(Vector2f::new(size.0, size.1));
        r.set_fill_color(self.color);
        if self.border_size.is_some() {
            r.set_outline_thickness(self.border_size.unwrap() as f32);
            r.set_outline_color(self.border_color.unwrap_or(Color::WHITE));
        }
        window.draw(&r);
    }
}


/// This struct stores a circle that
/// can be rendered to the window.
pub struct Circle {
    pos: Vector<u16>,
    radius: u16,
    color: Color,
    border_size: Option<u16>,
    border_color: Option<Color>,
}
impl Circle {
    /// Create a new rectangle that can be rendered to the window.
    /// 
    /// ### Arguments
    /// * `pos`: [Into]<[Vector]<[u16]>> - The topleft position.
    /// * `radius`: [u16] - The circle radius.
    /// * `color`: [Color] - The fill color.
    pub fn new<V: Into<Vector<u16>>>(pos: V, radius: u16, color: Color) -> Circle {
        Circle { pos: pos.try_into().unwrap(), radius, color, border_size: None, border_color: None }
    }

    /// Creates a border around the rect.
    pub fn border(&mut self, border_size: u16, border_color: Color) {
        self.border_size = Some(border_size);
        self.border_color = Some(border_color);
    }

    /// Renders the circle to the given window.
    pub fn render(&self, window: &mut RenderWindow) {
        let mut c = CircleShape::new(self.radius as f32, 30);
        let pos = (self.pos - self.radius).cast::<f32>();

        c.set_position(Vector2f::new(pos.0, pos.1));
        c.set_fill_color(self.color);
        if self.border_size.is_some() {
            c.set_outline_thickness(self.border_size.unwrap() as f32);
            c.set_outline_color(self.border_color.unwrap_or(Color::WHITE));
        }
        window.draw(&c);
    }
}
