use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};
use crate::Vector;
use crate::err;


pub struct Window {
    title: String,
    size: Vector<u16>,
    pos: Vector<u16>
}


pub fn new(title: &str, size: Vector<u16>, pos: Option<Vector<u16>>) -> Result<Window, err::Error> {
    let win_pos = pos.unwrap_or(Vector(150, 150));

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    window.set_title(title);
    window.set_inner_size(size.logical_size());
    window.set_outer_position(win_pos.logical_pos());

    /*
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
    */

    Ok(Window { title: title.to_string(), size, pos: win_pos })
}
