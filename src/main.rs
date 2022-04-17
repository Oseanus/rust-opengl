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

    // Shaders
    let vertex_shader_src = r#"
        #version 430

        in vec2 position;
        uniform float t;

        void main() {
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    
    // Define vertice
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    // 5. Run event_loop infinitely
    let mut t: f32 = -0.5;
    event_loop.run(move |event, _, control_flow| {
        
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Update t
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        // Draw triangle
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform!{ t: t },
                    &Default::default()).unwrap();
        target.finish().unwrap();
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2]
}

implement_vertex!(Vertex, position);