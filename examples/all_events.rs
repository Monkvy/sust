
struct App;
impl sust::App for App {
    fn start(&mut self, state: sust::State) {
        println!("Hi, my title is {}", state.config.title)
    }

    fn events(&mut self, state: sust::State, event: sust::Event) -> bool {
        match event {
            sust::Event::KeyPressed { code, alt: _, ctrl: _, shift: _, system: _ } if code == sust::Key::Space => {
                println!("Mouse at {:?}", state.mouse.pos);
                true
            },
            _ => false,
        }
    }

    fn gui(&mut self, state: sust::State, ctx: &sust::gui::Context) {
        sust::gui::Window::new("Hello").show(ctx, |ui| {
            ui.label(String::from("Fps: ") + state.fps.to_string().as_str());
        });
    }

    fn update(&mut self, _: sust::State) {}

    fn render(&self, _: sust::State, window: &mut sust::RenderWindow) {
        sust::shape::Line::new((100, 125), (600, 900), sust::Color::RED, 5).render(window);
    }
}

fn main() {
    sust::run(sust::Config {
        title: String::from("Sussy window uwu"),
        .. sust::Config::default()
    }, sust::GuiConfig::default(), App);
}
