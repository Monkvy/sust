use crate::{Vector, Style, gui};


/// This struct stores the necessary window attributes.
#[derive(Clone)]

#[derive(Debug)]
pub struct Config {
    pub title: String,
    pub size: Vector<u16>,
    pub pos: Vector<i32>,
    pub style: Style,
    pub max_fps: u32
}
impl Config {
    pub fn default() -> Config {
        Config { 
            title: String::from("Sussy window"), 
            size: Vector(1280, 720), 
            pos: Vector(150, 150), 
            style: Style::CLOSE,
            max_fps: 144
        }
    }
}


/// Stores all font sizes & families
/// of all different text types.
#[derive(Debug, Clone)]
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
