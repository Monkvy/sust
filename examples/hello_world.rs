
struct App;
impl sust::App for App {
    fn events(&mut self, _event: sust::Event) -> bool { false }
    fn update(&mut self, _dt: f32) {}
    fn gui(&mut self, _ctx: &sust::gui::Context) {}
    fn render(&self, _window: &mut sust::RenderWindow) {}
}

fn main() {
    sust::run("Hello world", 144, sust::Config::size((800, 600)),sust::GuiConfig::default(), App);
}
