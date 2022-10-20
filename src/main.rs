

struct App;
impl sust::App for App {
    fn events(&mut self, _event: sust::Event) -> bool { false }

    fn update(&mut self, dt: f32) {
        println!("{}", 1. / dt);
    }
    fn gui(&mut self, _ctx: &sust::gui::Context) {}
    fn render(&self, _window: &mut sust::RenderWindow) {}
}


fn main() {
    sust::run("Sust window", std::u32::MAX, sust::Config::size((1200, 800)), sust::GuiConfig::default(), App);
}
