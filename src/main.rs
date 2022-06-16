use std::collections::HashMap;
use std::thread;
use std::time;

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
    for i in 0..4 {         // TODO: Change to 10 and 20
        for j in 0..5 {
            let window = Window::new(&event_loop).unwrap();
            window.set_inner_size(PhysicalSize::new(50, 35));
            window.set_outer_position(PhysicalPosition::new((j*70)+30, (i*85)+70));
            window.set_decorations(false);
    
            windows.insert(iter, window);
            iter += 1;
        }
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // TODO: Loop through the hashmap and flash the windows for 1s
        //       for testing purposes.

        // NOTE: Test code, works, got to move a window and move it back
        // let key: u32 = 1;                                                Creates variables to work with the window
        // let window_1 = windows.get_key_value(&key).unwrap().1;           Gets the window out of the hashmap
        // let window_pos = &window_1.outer_position().unwrap();            Saves the position of the window

        // window_1.set_outer_position(PhysicalPosition::new(1000, 500));   Sets position of the window to some place

        // thread::sleep(time::Duration::from_millis(1000));                Waits for 1 second

        // window_1.set_outer_position(*window_pos);                        Sets position of the window to original place

        // thread::sleep(time::Duration::from_millis(2000));                Waits for 2 seconds

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