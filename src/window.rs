use crate::{
    gui,
    Vector, App,
    Config, GuiConfig, State, MouseState,
    Event, RenderWindow, Color,
};
use sfml::{
    system::{Vector2, Clock},
    graphics::RenderTarget
};
use egui_sfml::SfEgui;


/// Runs the main loop of the window.
/// 
/// ### Arguments
/// * `config`: [Config] - The window config.
/// * `gui_config` [GuiConfig] - The Gui config.
/// * `app`: mut [App] - Your application struct.
pub fn run<T: App> (config: Config, gui_config: GuiConfig, mut app: T) {

    // Window
    let mut window = RenderWindow::new(
        config.size,
        config.title.as_str(),
        config.style,
        &Default::default(),
    );
    window.set_position(Vector2::new(config.pos.0, config.pos.1));
    window.set_framerate_limit(config.max_fps);

    // State
    let mut gui = SfEgui::new(&window);
    let mut dt_clock = Clock::start();
    let mut state = &mut State{
        config, gui_config: gui_config.clone(),
        mouse: MouseState{button: None, pos: Vector(0, 0)},
        dt: 0.,
        fps: 0.
    };

    // Start event
    app.start(state.clone());

    // Main loop
    while window.is_open() {

        // Events
        while let Some(event) = window.poll_event() {
            gui.add_event(&event);
            match event {
                Event::Closed => window.close(),

                // Set state.mouse
                Event::MouseButtonPressed { button, x, y } => {
                    state.mouse.button = Some(button);
                    state.mouse.pos = Vector(x as u16, y as u16);
                }
                Event::MouseButtonReleased { button: _, x, y } => {
                    state.mouse.button = None;
                    state.mouse.pos = Vector(x as u16, y as u16);
                }
                Event::MouseMoved { x, y } => {
                    state.mouse.pos = Vector(x as u16, y as u16);
                }

                // Call app.events
                _ => { if app.events(state.clone(), event) { break; } }
            }
        }

        // Gui
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
            app.gui(state.clone(), ctx);
        });

        // Update
        state.dt = dt_clock.restart().as_seconds();
        state.fps = 1. / state.dt;
        app.update(state.clone());
        
        // Render
        window.clear(Color::BLACK);
        app.render(state.clone(), &mut window);
        gui.draw(&mut window, None);
        window.display();
    }
}
