use sust::Grid;


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
    fn border(&self) -> (u16, sust::Color) {
        (0, sust::Color::TRANSPARENT)
    }
}

struct App{ grid: Grid<Cell> }
impl sust::App for App {
    fn events(&mut self, event: sust::Event) -> bool {
        match event {
            sust::Event::MouseButtonPressed { button, x, y } => {
                if button == sust::mouse::Button::Left {
                    let cell = &mut self.grid.map_vec((x as u16, y as u16)).unwrap();
                    cell.active = !cell.active;
                }
                true
            }
            _ => false,
        }
    }

    fn update(&mut self, _dt: f32) {}
    fn gui(&mut self, _ctx: &sust::gui::Context) {}
    fn render(&self, window: &mut sust::RenderWindow) {
        self.grid.render(window);
    }

}


fn main() {
    let grid = sust::Grid::new(50, 50, 10);
    sust::run("Sust window", grid.win_conf(), sust::GuiConfig::default(), App {grid});
}
