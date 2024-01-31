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

mod teapot;

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
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

    let image = image::load(
        std::io::Cursor::new(&include_bytes!("../assets/meandcat.jpg")[..]),
        image::ImageFormat::Jpeg,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

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
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 1.0, 1.0);

                let matrix = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ];

                frame
                    .draw(
                        (&positions, &normals),
                        &indices,
                        &program,
                        &uniform! { matrix: matrix },
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
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);
