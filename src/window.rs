use crate::{Vector, App, Style, Event, RenderWindow, Color, gui};
use sfml::{
    system::Vector2,
    graphics::RenderTarget
};
use egui_sfml::SfEgui;


/// This struct stores the necessary window attributes.
pub struct Config {
    size: Vector<u16>,
    pos: Vector<i32>,
    style: Style
}
impl Config {
    /// Create a config object based on all options.
    /// 
    /// ### Arguments
    /// * `size`: [Into]<[Vector]<[u16]>> - The window size.
    /// * `pos`: [Into]<[Vector]<[u16]>> - The window pos.
    /// * `style`: Style - Additional sfml window style options.
    pub fn new<Vu16: Into<Vector<u16>>, Vi32: Into<Vector<i32>>>(size: Vu16, pos: Vi32, style: Style) -> Config {
        Config { size: size.try_into().unwrap(), pos: pos.try_into().unwrap(), style }
    }

    /// Create a config object based on the size.
    /// All other attributes will be set to default.
    /// 
    /// ### Arguments
    /// * `size`: [Into]<[Vector]<[u16]>> - The window size.
    pub fn size<V: Into<Vector<u16>>>(size: V) -> Config {
        Config { size: size.try_into().unwrap(), pos: Vector(150, 150), style: Style::CLOSE }
    }

    /// Create a config object based on the size & position.
    /// All other attributes will be set to default.
    /// 
    /// ### Arguments
    /// * `size`: [Into]<[Vector]<[u16]>> - The window size.
    /// * `pos`: [Into]<[Vector]<[u16]>> - The window pos.
    pub fn pos<Vu16: Into<Vector<u16>>, Vi32: Into<Vector<i32>>>(size: Vu16, pos: Vi32) -> Config {
        Config { size: size.try_into().unwrap(), pos: pos.try_into().unwrap(), style: Style::CLOSE }
    }

    /// Create a config object.
    /// The window size will be calculated automatically based on the grid size & scale.
    pub fn grid(rows: u16, cols: u16, scale: u16) -> Config {
        Config { size: Vector(cols * scale, rows * scale), pos: Vector(150, 150), style: Style::CLOSE }
    }
}


#[derive(Clone)]
pub struct GuiConfig {
    pub heading: gui::FontId,
    pub heading_2: gui::FontId,
    pub context: gui::FontId,
    pub body: gui::FontId,
    pub monospace: gui::FontId,
    pub button: gui::FontId,
    pub small: gui::FontId
}
impl GuiConfig {
    pub fn default() -> GuiConfig {
        GuiConfig { 
            heading:   gui::FontId::new(25., gui::FontFamily::Proportional),
            heading_2: gui::FontId::new(22., gui::FontFamily::Proportional),
            context:   gui::FontId::new(22., gui::FontFamily::Proportional),
            body:      gui::FontId::new(22., gui::FontFamily::Proportional),
            monospace: gui::FontId::new(22., gui::FontFamily::Proportional),
            button:    gui::FontId::new(22., gui::FontFamily::Proportional),
            small:     gui::FontId::new(22., gui::FontFamily::Proportional)
        }
    }
}


/// Runs the main loop of the window.
/// 
/// ### Arguments
/// * `title`: &[str] - The window title.
/// * `config`: [Config] - The window config.
/// * `app`: mut [App] - Your application struct.
pub fn run<T: App> (title: &str, config: Config, gui_config: GuiConfig, mut app: T) {
    let mut window = RenderWindow::new(
        config.size,
        title,
        config.style,
        //Style::CLOSE,
        &Default::default(),
    );
    window.set_position(Vector2::new(config.pos.0, config.pos.1));
    let mut gui = SfEgui::new(&window);

    while window.is_open() {

        // Events
        while let Some(event) = window.poll_event() {
            gui.add_event(&event);
            match event {
                Event::Closed => window.close(),
                _ => { if app.events(event) {break;} }
            }
        }
        gui.do_frame(|ctx| {
            let mut style = (*ctx.style()).clone();
            style.text_styles = [
                (gui::TextStyle::Heading,                 gui_config.clone().heading),
                (gui::TextStyle::Name("Heading2".into()), gui_config.clone().heading_2),
                (gui::TextStyle::Name("Context".into()),  gui_config.clone().context),
                (gui::TextStyle::Body,                    gui_config.clone().body),
                (gui::TextStyle::Monospace,               gui_config.clone().monospace),
                (gui::TextStyle::Button,                  gui_config.clone().button),
                (gui::TextStyle::Small,                   gui_config.clone().small),
            ].into();
            ctx.set_style(style);
            app.gui(ctx);
        });

        // TODO: Calculate dt
        app.update(0.);
        window.clear(Color::BLACK);
        app.render(&mut window);
        gui.draw(&mut window, None);
        window.display();
    }
}
