mod node;
mod node_map;
mod rectangle;
mod triangle;
mod utils;
mod vert;

use crate::node_map::NodeMap;
use crate::rectangle::buffers_from_rec;

#[macro_use]
extern crate glium;

const HEIGHT: f32 = 1080.0;
const WIDTH: f32 = 1920.0;
const SIZE: f32 = 10.0;
const NOISE: bool = true;
const ACTIVE_COLOR: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);

fn main() {
    use glium::{glutin, Surface};

    // Glium setup
    let event_loop = glutin::event_loop::EventLoop::new();
    let fullscreen = Some(glutin::window::Fullscreen::Borderless(None));
    let wb = glutin::window::WindowBuilder::new().with_fullscreen(fullscreen);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 1.0, 1.0, 1.0);
            }
        "#;
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut node_map = NodeMap::for_screen();

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color(
            ACTIVE_COLOR.0,
            ACTIVE_COLOR.1,
            ACTIVE_COLOR.2,
            ACTIVE_COLOR.3,
        );

        node_map.calculate();

        for row in node_map.nodes.iter_mut() {
            for node in row.iter_mut() {
                if !node.will_be_on {
                    node.turn_off();
                    continue;
                }

                node.turn_on();

                let rec_buffers = buffers_from_rec(node.rectangle, &display);

                for buffer in rec_buffers {
                    target
                        .draw(
                            &buffer,
                            &indices,
                            &program,
                            &glium::uniforms::EmptyUniforms,
                            &Default::default(),
                        )
                        .unwrap();
                }
            }
        }

        target.finish().unwrap();
    });
}
