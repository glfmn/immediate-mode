#[macro_use]
extern crate glium;
extern crate immediate_mode;

use glium::{glutin, Surface};
use immediate_mode::{text::Texture, Color, Input, Renderer, Theme, Vec2, UI};

const VERT_SHADER_SRC: &str = r#"
#version 140

in vec2 pos;
in vec2 uv;
in vec4 color;

out vec4 f_color;
out vec2 f_uv;

uniform mat4 u_view;

void main() {
    gl_Position = u_view * vec4(pos, 0.0, 1.0);
    f_uv = uv;
    f_color = color;
}
"#;

const FRAG_SHADER_SRC: &str = r#"
#version 140

in vec4 f_color;
in vec2 f_uv;

out vec4 color;

void main() {
    color = f_color;
}
"#;

#[derive(Copy, Clone, Debug)]
struct Vert {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

impl From<([f32; 2], [f32; 2], [u8; 4])> for Vert {
    fn from((pos, uv, color): ([f32; 2], [f32; 2], [u8; 4])) -> Self {
        let color = [
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
            color[3] as f32 / 255.0,
        ];
        Vert { pos, uv, color }
    }
}

implement_vertex!(Vert, pos, uv, color);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("immediate-mode");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut frame: usize = 0;

    let program =
        glium::Program::from_source(&display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap();

    let mut ui: UI<Vert> = UI::new(Input::new((0, 0), false));
    let mut cursor_pos = glutin::dpi::PhysicalPosition::new(0.0, 0.0);
    let mut cursor_down = false;
    event_loop.run(move |event, _, control_flow| {
        use glutin::event::{Event, StartCause, WindowEvent};
        use glutin::event_loop::ControlFlow;

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    cursor_pos = position;
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    use glutin::event::{ElementState, MouseButton};

                    if button == MouseButton::Left {
                        cursor_down = state == ElementState::Pressed;
                    }
                }
                _ => return,
            },
            // Limit frame rate
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        frame += 1;

        let (width, height) = display.get_framebuffer_dimensions();
        let scale_factor = display.gl_window().window().scale_factor();

        let id = ui.calculate_id(0);
        ui.set_active(id);
        ui.draw(|data| {
            data.tri_multicolor(
                (Vec2::new(0.0, 0.0), Theme::LIGHT.element),
                (Vec2::new(110.0, 0.0), Theme::LIGHT.fg),
                (Vec2::new(0.0, 110.0), Theme::LIGHT.bg_overlay),
            );
        });
        let e = ui.event(id, (Vec2::new(0.0, 0.0), Vec2::new(110.0, 110.0)));
        eprintln!("{:?}", e);

        let renderer = ui.finish_frame();

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let uniforms = uniform!(
            u_view: {
                let origin = (0.0, 0.0);
                let l = origin.0; // left
                let r = origin.0 + width as f32 / scale_factor as f32;
                let t = origin.1; // top
                let b = origin.1 + height as f32 / scale_factor as f32;
                [
                    [2.0 / (r - l), 0.0, 0.0, 0.0],
                    [0.0, 2.0 / (t - b), 0.0, 0.0],
                    [0.0, 0.0, -1.0, 0.0],
                    [(r + l) / (l - r), (t + b) / (b - t), 0.0, 1.0],
                ]
            }
        );

        let vbo = glium::VertexBuffer::new(&display, renderer.verts()).unwrap();
        let ibo = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            renderer.indicies(),
        )
        .unwrap();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(&vbo, &ibo, &program, &uniforms, &draw_params)
            .unwrap();

        target.finish().unwrap();

        renderer.next_frame(Input::new(
            (cursor_pos.x as u32, cursor_pos.y as u32),
            cursor_down,
        ));
    });
}
