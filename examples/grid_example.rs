
#[derive(Clone)]
struct Cell{pub active: bool}
impl sust::Cell for Cell {
    fn default() -> Cell { Cell {active: true} }
    fn color(&self) -> sust::Color {
        match self.active {
            true => sust::Color::WHITE,
            false => sust::Color::BLACK,
        }
    }
}

struct App{ grid: sust::Grid<Cell> }
impl sust::App for App {
    fn update(&mut self, state: sust::State) {
        if state.mouse.button == Some(sust::mouse::Button::Left) {
            let cell = self.grid.map_vec(state.mouse.pos);
            if cell.is_some() {
                let c = cell.unwrap();
                c.active = !c.active;
            }
        }
    }
    fn render(&self, _state: sust::State, window: &mut sust::RenderWindow) {
        self.grid.render(window);
    }
}


fn main() {
    let grid = sust::Grid::new(50, 50, 10);
    sust::run(grid.win_config(String::from("Grid"), None), sust::GuiConfig::default(), App {grid});
}
