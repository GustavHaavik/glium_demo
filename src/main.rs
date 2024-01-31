use glium::backend::glutin::SimpleWindowBuilder;
use glium::index::NoIndices;
use glium::index::PrimitiveType;
use glium::Program;
use glium::Surface;
use glium::VertexBuffer;
use winit::event::Event;
use winit::event::WindowEvent;

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = NoIndices(PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 150

        in vec2 position;

        uniform float x;

        void main() {
            vec2 pos = position;
            pos.x += x;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();

    let mut t: f32 = 0.0;

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
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

                let x_offset = t.sin() * 0.5;

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);
                target
                    .draw(
                        &vertex_buffer,
                        &indices,
                        &program,
                        &uniform! { x: x_offset },
                        &Default::default(),
                    )
                    .unwrap();
                target.finish().unwrap();
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
}
implement_vertex!(Vertex, position);
