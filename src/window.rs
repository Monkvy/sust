use crate::Vector;
use crate::err;


pub struct Window {
    title: String,
    size: Vector<u16>,
    pos: Vector<u16>,
    winit_win: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>
}


pub fn new(title: &str, size: Vector<u16>, pos: Option<Vector<u16>>) -> Result<Window, err::Error> {
    let win_pos = pos.unwrap_or(Vector(150, 150));

    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new();
    let winit_win = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
    
    winit_win.set_title(title);
    winit_win.set_inner_size(size.logical_size());
    winit_win.set_outer_position(win_pos.logical_pos());

    Ok(Window { 
        title: title.to_string(), 
        size, 
        pos: win_pos,
        winit_win,
        event_loop
    })
}


impl Window {
    pub fn run<T: Fn(winit::event::Event<()>) + 'static>(self, event_fn: T) -> Result<(), err::Error> {
        self.event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.winit_win.id() => match event {
                winit::event::WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => {}
            },
            _ => { event_fn(event) }
        });
    }
}
