

struct App {a: f32}
impl sust::App for App {
    fn events(&mut self, _event: sust::Event) -> bool { false }

    fn update(&mut self, _dt: f32) {self.a+=0.1}
    fn render(&self, window: &mut sust::RenderWindow) {
        sust::shape::Line::new((50, 50), (150, 100), sust::Color::RED, 2).render(window);
        sust::shape::Line::from_vec((50, 50), (150, 100), sust::Color::BLUE, 2).render(window);
        sust::shape::Rect::new((50, 50), (4, 4), sust::Color::WHITE).render(window);
    }
}


fn main() {
    sust::run("Sust window", sust::Config::size((800, 600)), App{a:0.});
}
