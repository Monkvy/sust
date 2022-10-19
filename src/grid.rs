use crate::{WindowConfig, Color, Vector, RenderWindow, Rect};


pub trait Cell {
    fn default() -> Self;

    fn color(&self) -> Color {
        Color::BLACK
    }

    fn border(&self) -> (u16, Color) {
        (0, Color::TRANSPARENT)
    }
}

#[derive(Clone)]
pub struct Grid<T: Cell + Clone> {
    rows: u16,
    cols: u16,
    scale: u16,
    cells: Vec<Vec<T>>
}

impl<T: Cell + Clone> Grid<T> {
    pub fn new(rows: u16, cols: u16, scale: u16) -> Grid<T> {
        Grid::<T> { rows, cols, scale, cells: vec![vec![T::default(); cols as usize]; rows as usize] }
    }

    pub fn win_conf(&self) -> WindowConfig {
        WindowConfig::Grid { rows: self.rows, cols: self.cols, scale: self.scale }
    }

    pub fn get_size(&self) -> Vector<u16> {
        Vector(self.rows, self.cols)
    }

    pub fn get_scale(&self) -> u16 {
        self.scale
    }

    pub fn map_vec<V: Into<Vector<u16>>>(&mut self, vec: V) -> Option<&mut T> {
        let v: Vector<u16> = vec.try_into().unwrap();
        let pos = v / self.scale;
        
        if pos.1 >= self.rows || pos.0 >= self.cols { return None }
        Some(&mut self.cells[pos.1 as usize][pos.0 as usize])
    }

    pub fn render(&self, window: &mut RenderWindow) {
        for (r, row) in self.cells.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {

                let mut r = Rect::new((c as u16 * self.scale, r as u16 * self.scale), (self.scale, self.scale), cell.color());
                r.border(cell.border().0, cell.border().1);

                r.render(window);
            }
        }
    }
}
