#[macro_use]
extern crate glium;
extern crate immediate_mode;

use glium::{glutin, Surface};
use immediate_mode::{theme, Color, DrawData, Theme, Vec2};

const VERT_SHADER_SRC: &str = r#"
#version 140

in vec2 pos;
in vec2 uv;
in vec4 color;
out vec4 f_color;
out vec2 tex;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
    tex = uv;
    f_color = color / 255.0;
}
"#;

const FRAG_SHADER_SRC: &str = r#"
#version 140

out vec4 color;
in vec2 tex;
in vec4 f_color;
void main() {
    color = f_color;
}
"#;

#[derive(Copy, Clone, Debug)]
struct Vert {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [u8; 4],
}

impl From<([f32; 2], [f32; 2], [u8; 4])> for Vert {
    fn from((pos, uv, color): ([f32; 2], [f32; 2], [u8; 4])) -> Self {
        Vert { pos, uv, color }
    }
}

implement_vertex!(Vert, pos, uv, color);

fn colors(draw: &mut DrawData<Vert>, bg: Color, fg: &[Color], x1: f32, x2: f32, y1: f32, y2: f32) {
    draw.rect(bg, Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 });

    let x_min = x1.min(x2);
    let x_max = x1.max(x2);
    let y_min = y1.min(y2);
    let y_max = y1.max(y2);

    let count = fg.len() + 4;
    let height = (y_max - y_min) / count as f32;

    for (n, color) in fg.iter().enumerate() {
        let y = (n as f32) * height;
        draw.rect(
            *color,
            Vec2 {
                x: x_min + 0.1,
                y: y_max - height - y,
            },
            Vec2 {
                x: x_max - 0.1,
                y: (y_max - 2.0 * height) - y,
            },
        );
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("immediate-mode");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let program =
        glium::Program::from_source(&display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap();

    let mut frame: usize = 0;

    let dark_colors = [
        Theme::DARK.fg,
        Theme::DARK.fg_disabled,
        Theme::DARK.bg_highlight,
        Theme::DARK.bg_child,
        Theme::DARK.border,
        Theme::DARK.element,
        Theme::DARK.fg_selected,
        Theme::DARK.selected,
        Theme::DARK.hover,
        Theme::DARK.active,
    ];

    let dark_bgs = [Theme::DARK.bg, Theme::DARK.bg_child];

    let light_colors = [
        Theme::LIGHT.fg,
        Theme::LIGHT.fg_disabled,
        Theme::LIGHT.bg_highlight,
        Theme::LIGHT.bg_child,
        Theme::LIGHT.border,
        Theme::LIGHT.element,
        Theme::LIGHT.fg_selected,
        Theme::LIGHT.selected,
        Theme::LIGHT.hover,
        Theme::LIGHT.active,
    ];

    let light_bgs = [Theme::LIGHT.bg, Theme::LIGHT.bg_child];

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
        println!("frame: {}", frame);

        let mut draw = DrawData {
            verts: Vec::with_capacity((light_colors.len() + 2) * 8),
            indicies: Vec::with_capacity((light_colors.len() + 2) * 12),
        };

        colors(
            &mut draw,
            dark_bgs[(frame / 150) % dark_bgs.len()],
            &dark_colors,
            -1.0,
            0.0,
            1.0,
            -1.0,
        );
        colors(
            &mut draw,
            light_bgs[(frame / 150) % dark_bgs.len()],
            &light_colors,
            0.0,
            1.0,
            1.0,
            -1.0,
        );

        draw.rect(
            Theme::LIGHT.bg_overlay,
            Vec2 { x: 0.5, y: -1.0 },
            Vec2 { x: 1.0, y: 1.0 },
        );
        draw.rect(
            Theme::DARK.bg_overlay,
            Vec2 { x: -0.5, y: -1.0 },
            Vec2 { x: -1.0, y: 1.0 },
        );

        let mut xs = Vec::with_capacity(100);
        for x in 0..100 {
            let x = (x as f32) / 100.0;
            let pos = Vec2 {
                x: 2.0 * x - 1.0,
                y: (10.0 * x + (frame as f32) / 10.0).sin(),
            };
            xs.push(pos);
        }

        draw.polyline(theme::RED, 0.005, &xs);

        let vbo = glium::VertexBuffer::new(&display, draw.verts.as_slice()).unwrap();
        let ibo = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            draw.indicies.as_slice(),
        )
        .unwrap();

        let mut target = display.draw();
        let clear: [f32; 4] = Theme::LIGHT.bg.into();
        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };
        target.clear_color(clear[0], clear[1], clear[2], clear[3]);
        target
            .draw(
                &vbo,
                &ibo,
                &program,
                &glium::uniforms::EmptyUniforms,
                &draw_params,
            )
            .unwrap();
        target.finish().unwrap();
    });
}
