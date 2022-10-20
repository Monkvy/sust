
struct App;
impl sust::App for App {}

fn main() {
    sust::run(sust::Config {
        title: String::from("Sussy window uwu"),
        .. sust::Config::default()
    }, sust::GuiConfig::default(), App);
}
