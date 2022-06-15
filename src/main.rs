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

    let mut iter: u32 = 0;
    for i in 0..10 {
        for j in 0..20 {
            let window = Window::new(&event_loop).unwrap();
            window.set_inner_size(PhysicalSize::new(50, 35));
            window.set_outer_position(PhysicalPosition::new((j*70)+30, (i*85)+70));
            window.set_title(&iter.to_string());
            window.set_decorations(false);
    
            windows.insert(window.id(), window);
            iter += 1;
        }
    }

    // println!("{:?}", windows);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

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