use std::collections::HashMap;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    dpi::{PhysicalPosition, PhysicalSize}
};

fn main() {
    let event_loop = EventLoop::new();
    let mut windows = HashMap::new();
    for i in 0..18 {
        for j in 0..10 {
            let window = Window::new(&event_loop).unwrap();
            window.set_inner_size(PhysicalSize::new(210, 210));
            window.set_outer_position(PhysicalPosition::new(i*210, j*200));
            window.set_decorations(false);
    
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
