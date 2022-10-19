use crate::{Config, Color, Vector, RenderWindow, Rect};


/// Use this trait on a struct to uses it
/// as cell inside a Grid.
/// 
/// ### Methods
/// * `default()` -> Self - The initial state of every cell.
/// * `color(&self)` -> Color - Change the cell color based on some conditions if you want.
/// * `border(&self)` -> (u16, Color) - Change the cell border if you want.
pub trait Cell {
    fn default() -> Self;
    fn color(&self) -> Color { Color::BLACK }
    fn border(&self) -> (u16, Color) { (0, Color::TRANSPARENT) }
}

/// A grid stores a 2d array of cells that 
/// are rendered to the window.
/// Call Grid::new to create one.
#[derive(Clone)]
pub struct Grid<T: Cell + Clone> {
    rows: u16,
    cols: u16,
    scale: u16,
    cells: Vec<Vec<T>>
}

impl<T: Cell + Clone> Grid<T> {
    /// Create a new grid that stores cells that are rendered to the window.
    /// The grid gets filled with the result of the Cell::default() function.
    pub fn new(rows: u16, cols: u16, scale: u16) -> Grid<T> {
        Grid::<T> { rows, cols, scale, cells: vec![vec![T::default(); cols as usize]; rows as usize] }
    }

    /// Get the window config based on the size & scale of the grid.
    pub fn win_conf(&self) -> Config {
        Config::grid(self.rows, self.cols, self.scale)
    }

    /// Get the rows & cols of this grid.
    pub fn get_size(&self) -> Vector<u16> {
        Vector(self.rows, self.cols)
    }

    /// Get the scale.
    pub fn get_scale(&self) -> u16 {
        self.scale
    }

    /// Map the given vector onto the grid based on the scale
    /// and get the corresponding cell.
    /// 
    /// ### Arguments
    /// * `vec`: [Into]<[Vector<[u16]>]> - The position to map.
    ///
    /// ### Returns
    /// A [Option]<&mut [Cell]> containing the corresponding cell.
    pub fn map_vec<V: Into<Vector<u16>>>(&mut self, vec: V) -> Option<&mut T> {
        let v: Vector<u16> = vec.try_into().unwrap();
        let pos = v / self.scale;

        if pos.1 >= self.rows || pos.0 >= self.cols { return None }
        Some(&mut self.cells[pos.1 as usize][pos.0 as usize])
    }

    /// Renders the grid to the given window.
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
