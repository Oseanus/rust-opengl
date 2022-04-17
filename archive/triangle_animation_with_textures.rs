#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;
use image::{load, ImageFormat};

#[allow(unused_imports)]
use glium::{glutin, Surface};

fn main() {
    // 1. Create event_loop
    let event_loop = glutin::event_loop::EventLoop::new();

    // 2. Specify window parameters
    let wb = glutin::window::WindowBuilder::new();

    // 3. Specify OpenGL attributes
    let cb = glutin::ContextBuilder::new();

    // 4. Create OpenGL window
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Define vertice
    let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    
    // Define vertex shader and fragment shader
    let vertex_shader_src = r#"
        #version 430

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
        
    let fragment_shader_src = r#"
        #version 430

        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;
        
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Load textures
    let image = load(Cursor::new(&include_bytes!("../resources/images/opengl-logo.png")),
                                                              ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, raw_image).unwrap();
    
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

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        
        // Update t
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }
        
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        
        // Define matrix for vertex shader
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };

            // Draw triangle
            target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
            target.finish().unwrap();
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);