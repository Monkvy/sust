
struct App;
impl sust::App for App {
    fn events(&mut self, _event: sust::Event) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    fn render(&self, _window: &mut sust::RenderWindow) {
}
}

fn main() {
    sust::run("Hello world", sust::Config::size((800, 600)), App);
}