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
    for i in 0..10 {
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

        // NOTE: formula for advancing frame by frame through the 2D array.
        //       y_index + (10 * frame[that starts from 0])
        //
        //       Will have to nest this code below in another for loop
        //       that is "for frames in length/10".

        for i in &windows {
            let window_1 = i.1;

            let index = *i.0 as usize;
            let y_index = &index % 20;
            let x_index = &index / 20;

            let window_pos = PhysicalPosition::new((x_index as u32 *70)+30, (y_index as u32 *85)+70);

            // println!("{:?}: ({:?}, {:?})", index, index1, index2);

            if test_array[x_index][y_index] == 1 {
                window_1.set_outer_position(PhysicalPosition::new(1700, 540));
            } else {
                window_1.set_outer_position(window_pos);
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