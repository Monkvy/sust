use crate::{Vector, Color};
use sfml::{
    graphics::{RenderWindow, RenderTarget, RectangleShape, Transformable, Shape, CircleShape},
    system::Vector2f
};


// Rect
pub struct Rect {
    pos: Vector<u16>,
    size: Vector<u16>,
    color: Color,
    border_size: Option<u16>,
    border_color: Option<Color>,
}
impl Rect {
    pub fn new<V: Into<Vector<u16>>>(pos: V, size: V, color: Color) -> Rect {
        Rect { pos: pos.try_into().unwrap(), size: size.try_into().unwrap(), color, border_size: None, border_color: None }
    }

    pub fn border(&mut self, border_size: u16, border_color: Color) {
        self.border_size = Some(border_size);
        self.border_color = Some(border_color);
    }

    pub fn center(&mut self, center: Vector<u16>) {
        self.pos = center - self.size / 2;
    }

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


// Circle
pub struct Circle {
    pos: Vector<u16>,
    radius: u16,
    color: Color,
    border_size: Option<u16>,
    border_color: Option<Color>,
}
impl Circle {
    pub fn new<V: Into<Vector<u16>>>(pos: V, radius: u16, color: Color) -> Circle {
        Circle { pos: pos.try_into().unwrap(), radius, color, border_size: None, border_color: None }
    }

    pub fn border(&mut self, border_size: u16, border_color: Color) {
        self.border_size = Some(border_size);
        self.border_color = Some(border_color);
    }

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
