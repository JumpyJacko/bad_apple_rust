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

    let test_array: [[u32; 20]; 10] = 
    [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    ];

    let mut iter: u32 = 0;
    for i in 0..10 {         // TODO: Change to 10 and 20
        for j in 0..20 {
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
        //       for testing purposes. DONE

        // NOTE: formula for advancing frame by frame through the 2D array.
        //       y_index + (10 * frame[that starts from 0])

        for i in &windows {
            let window_1 = i.1;
            let window_pos = &window_1.outer_position().unwrap();

            let index = *i.0 as usize;
            let y_index = (&index % 20);
            let x_index = (&index / 20);
            // println!("{:?}: ({:?}, {:?})", index, index1, index2);

            if test_array[x_index][y_index] == 1 {
                window_1.set_outer_position(PhysicalPosition::new(1700, 540));
            } else {
                window_1.set_outer_position(*window_pos);
            }
        }

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