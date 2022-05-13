#[macro_use]
extern crate glium;
extern crate image;

#[allow(unused_imports)]
use glium::{glutin, Surface};

mod teapot_chapter_7;

fn main() {
    // 1. Create event_loop
    let event_loop = glutin::event_loop::EventLoop::new();
    
    // 2. Specify window parameters
    let wb = glutin::window::WindowBuilder::new();

    // 3. Specify OpenGL attributes
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);

    // 4. Create OpenGL window
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
    // Define vertex shader and fragment shader
    let vertex_shader_src = r#"
        #version 430

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
    "#;
        
    let fragment_shader_src = r#"
        #version 430
        
        in vec3 v_normal;
        out vec4 color;

        uniform vec3 u_light;
        
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;
        
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Load teapot data
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();
    
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
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        
        // The direction of the light
        let light = [-1.0, 0.4, 0.9f32];
        
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspec_ratio = height as f32 / width as f32;
            
            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;
            
            let f = 1.0 / (fov / 2.0).tan();
            
            [
                [f * aspec_ratio,   0.0,        0.0,                                0.0],
                [       0.0,        f,          0.0,                                0.0],
                [       0.0,        0.0,    (zfar + znear) / (zfar - znear),        1.0],
                [       0.0,        0.0,    -(2.0 * zfar * znear) / (zfar - znear), 0.0]
                ]
            };
            
        // Define matrix for vertex shader
        let uniforms = uniform! {
            matrix: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [ t , 0.0, 2.0, 1.0f32],
            ],
            perspective: perspective,
            u_light: light
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
            
        // Draw triangle
        target.draw((&positions, &normals), &indices, &program, &uniforms, &params).unwrap();
        target.finish().unwrap();
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);