use crate::{mouse, Vector, Config, GuiConfig};


/// Stores the current mouse button & position.
/// If button == None, the pos == to the mouse position
/// of the last mouse press / release.
#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    pub button: Option<mouse::Button>,
    pub pos: Vector<u16>
}

/// Stores some live updated attributes of the window.
#[derive(Debug, Clone)]
pub struct State {
    pub config: Config,
    pub gui_config: GuiConfig,
    pub mouse: MouseState,
    pub dt: f32,
    pub fps: f32,
}
