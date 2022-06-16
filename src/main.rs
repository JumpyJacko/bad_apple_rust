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
    
            windows.insert(iter, window);
            iter += 1;
        }
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // NOTE: Test code, works, got the 2nd windows to go invisible
        // let key: u32 = 1;
        // let window_1 = windows.get_key_value(&key).unwrap().1;
        // window_1.set_visible(false); 

        // TODO: Loop through the hashmap and flash the windows for 1s
        //       for testing purposes.

        match event {
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CloseRequested => {
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