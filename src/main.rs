#[macro_use]
extern crate glium;

use std::time::{Instant, Duration};
use glium::glutin; // Window library
use glium::Surface;

fn main() {
    // 1. Create event_loop
    let mut event_loop = glutin::event_loop::EventLoop::new();
    
    // 2. Specify window parameters
    let wb = glutin::window::WindowBuilder::new();

    // 3. Specify OpenGL attributes
    let cb = glutin::ContextBuilder::new();

    // 4. Create OpenGL window
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // 5. Run event_loop infinitely
    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match  ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}