use glium::backend::glutin::SimpleWindowBuilder;
use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::Program;
use glium::Surface;
use glium::VertexBuffer;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyEvent;
use winit::event::WindowEvent;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let shape = vec![
        Vertex {
            position: [-0.5, -0.5],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.5],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5],
            color: [0.0, 0.0, 1.0],
        },
    ];

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = NoIndices(PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 150

        in vec2 position;
        in vec3 color;      // our new attribute
        out vec3 vertex_color;

        uniform mat4 matrix;

        void main() {
            vertex_color = color; // we need to set the value of each `out` variable.
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);   // We need an alpha value as well
        }
    "#;

    let program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = 0.0;

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                state: ElementState::Pressed,
                                repeat: false,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Draw triangle
                t += 0.08;

                let x = t.sin() * 0.5;

                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 1.0, 1.0);

                let uniforms = uniform! {
                    matrix: [
                        [ 1.0, 0.0, 0.0, 0.0 ],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32],
                    ]
                };

                frame
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();

                frame.finish().unwrap();
            }
            // show_image::event::Event::RedrawEventsCleared => {
            //     window.request_redraw();
            // }
            _ => (),
        }
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);
