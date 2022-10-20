
struct App;
impl sust::App for App {
    fn events(&mut self, _state: sust::State, _event: sust::Event) -> bool { false }
    fn update(&mut self, _state: sust::State) {}
    fn render(&self, _state: sust::State, _window: &mut sust::RenderWindow) {}
}

fn main() {
    sust::run( sust::Config {
        title: String::from("Sust window"),
        .. sust::Config::default()
    }, sust::GuiConfig::default(), App);
}
