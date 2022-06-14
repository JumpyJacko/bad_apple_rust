use std::collections::HashMap;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
    dpi::{PhysicalPosition, PhysicalSize}
};

fn main() {
    let event_loop = EventLoop::new();
    let mut windows = HashMap::new();
    for i in 0..13 {
        for j in 0..7 {
            let window = Window::new(&event_loop).unwrap();
            window.set_inner_size(PhysicalSize::new(300, 300));
            window.set_outer_position(PhysicalPosition::new(i*300, j*290));
    
            windows.insert(window.id(), window);
        }
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {event, window_id} => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("{:?} recieved request to close", window_id);
                        windows.remove(&window_id);

                        if windows.is_empty() {
                            *control_flow = ControlFlow::Exit
                        }
                    },
                    _ => ()
                }
            },
            _ => (),
        }
    });
}
