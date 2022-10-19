use sust::Grid;


#[derive(Clone)]
struct Test{pub x: bool}

impl sust::Cell for Test {
    fn default() -> Test {
        Test { x: true }
    }

    fn color(&self) -> sust::Color {
        match self.x {
            true => sust::Color::WHITE,
            false => sust::Color::BLACK,
        }
    }

    fn border(&self) -> (u16, sust::Color) {
        (0, sust::Color::TRANSPARENT)
    }
}

struct App{ grid: Grid<Test> }
impl sust::App for App {
    fn events(&mut self, event: sust::Event) -> bool {
        match event {
            sust::Event::KeyPressed { code, alt: _, ctrl: _, shift: _, system: _ } => {
                println!("{:?}", code);
                true
            },
            sust::Event::MouseButtonPressed { button, x, y } => {
                if button == sust::mouse::Button::Left {
                    let cell = &mut self.grid.map_vec((x as u16, y as u16)).unwrap();
                    cell.x = !cell.x;
                }
                true
            }
            _ => false,
        }
    }

    fn update(&mut self, _dt: f32) {

    }

    fn render(&self, window: &mut sust::RenderWindow) {
        self.grid.render(window);
    }
}



fn main() {
    let grid = sust::Grid::<Test>::new(50, 50, 10);
    sust::run("Sust window", grid.win_conf(), App {grid});
}
