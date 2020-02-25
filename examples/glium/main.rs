#[macro_use]
extern crate glium;
extern crate immediate_mode;

use glium::{glutin, Surface};
use immediate_mode::color::{self, Color, Theme};
use immediate_mode::math::Vec2;

static mut INDICIES: u32 = 0;

const VERT_SHADER_SRC: &str = r#"
#version 140

in vec2 position;
in vec2 uv;
in vec4 color;
out vec4 f_color;
out vec2 tex;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
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

#[derive(Copy, Clone)]
struct Vert {
    position: [f32; 2],
    uv: [f32; 2],
    color: [u8; 4],
}

implement_vertex!(Vert, position, uv, color);

impl From<Vec2> for Vert {
    fn from(vec: Vec2) -> Self {
        let color: [u8; 4] = Theme::LIGHT.bg.into();
        Vert {
            position: vec.into(),
            uv: [0.0, 0.0],
            color,
        }
    }
}

fn rect(a: Vec2, b: Vec2, color: Color) -> ([Vert; 4], [u32; 6]) {
    let verts = [
        Vert {
            position: a.into(),
            uv: [0.0, 0.0],
            color: color.into(),
        },
        Vert {
            position: [a.x, b.y],
            uv: [0.0, 0.0],
            color: color.into(),
        },
        Vert {
            position: [b.x, a.y],
            uv: [0.0, 0.0],
            color: color.into(),
        },
        Vert {
            position: b.into(),
            uv: [0.0, 0.0],
            color: color.into(),
        },
    ];
    let ind = unsafe {
        [
            INDICIES,
            INDICIES + 1,
            INDICIES + 2,
            INDICIES + 1,
            INDICIES + 2,
            INDICIES + 3,
        ]
    };
    unsafe {
        INDICIES += 4;
    }
    (verts, ind)
}

fn colors(bg: Color, fg: &[Color], x1: f32, x2: f32, y1: f32, y2: f32) -> (Vec<Vert>, Vec<u32>) {
    let mut verts = Vec::with_capacity(fg.len() * 4 + 4);
    let mut idxs = Vec::with_capacity(fg.len() * 6 + 6);
    let (vs, is) = rect(Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 }, bg);
    verts.extend(&vs);
    idxs.extend(&is);

    let x_min = x1.min(x2);
    let x_max = x1.max(x2);
    let y_min = y1.min(y2);
    let y_max = y1.max(y2);

    let count = fg.len() + 4;
    let height = (y_max - y_min) / count as f32;

    for (n, color) in fg.iter().enumerate() {
        let y = (n as f32) * height;
        let (vs, is) = rect(
            Vec2 {
                x: x_min + 0.1,
                y: y_max - height - y,
            },
            Vec2 {
                x: x_max - 0.1,
                y: (y_max - 2.0 * height) - y,
            },
            *color,
        );
        verts.extend(&vs);
        idxs.extend(&is);
    }
    (verts, idxs)
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let program =
        glium::Program::from_source(&display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap();

    let mut frame: usize = 0;

    let dark_colors = [
        color::WHITE,
        color::GRAY,
        color::BLACK,
        color::RED,
        color::dark::BRIGHT_RED,
        color::ORANGE,
        color::dark::BRIGHT_ORANGE,
        color::YELLOW,
        color::dark::BRIGHT_YELLOW,
        color::GREEN,
        color::dark::BRIGHT_GREEN,
        color::BLUE,
        color::dark::BRIGHT_BLUE,
        color::AQUA,
        color::dark::BRIGHT_AQUA,
        color::PURPLE,
        color::dark::BRIGHT_PURPLE,
        color::dark::FG1,
        color::dark::FG2,
        color::dark::FG3,
        color::dark::FG4,
        color::dark::BG1,
        color::dark::BG2,
        color::dark::BG3,
        color::dark::BG4,
    ];

    let dark_bgs = [
        color::dark::BGS,
        color::dark::BG,
        color::dark::BGH,
        color::dark::BG1,
        color::dark::BG2,
        color::dark::BG3,
        color::dark::BG4,
    ];

    let light_colors = [
        color::BLACK,
        color::GRAY,
        color::WHITE,
        color::RED,
        color::light::BRIGHT_RED,
        color::ORANGE,
        color::light::BRIGHT_ORANGE,
        color::YELLOW,
        color::light::BRIGHT_YELLOW,
        color::GREEN,
        color::light::BRIGHT_GREEN,
        color::BLUE,
        color::light::BRIGHT_BLUE,
        color::AQUA,
        color::light::BRIGHT_AQUA,
        color::PURPLE,
        color::light::BRIGHT_PURPLE,
        color::light::FG1,
        color::light::FG2,
        color::light::FG3,
        color::light::FG4,
        color::light::BG1,
        color::light::BG2,
        color::light::BG3,
        color::light::BG4,
    ];

    let light_bgs = [
        color::light::BGH,
        color::light::BG,
        color::light::BGS,
        color::light::BG1,
        color::light::BG2,
        color::light::BG3,
        color::light::BG4,
    ];

    event_loop.run(move |event, _, control_flow| {
        unsafe {
            INDICIES = 0;
        }
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

        let mut verts: Vec<Vert> =
            Vec::with_capacity((light_colors.len() + dark_colors.len()) * 4 + 8);
        let mut ids: Vec<u32> =
            Vec::with_capacity((light_colors.len() + dark_colors.len()) * 6 + 12);

        let (vs, is) = colors(
            dark_bgs[(frame / 150) % dark_bgs.len()],
            &dark_colors,
            -1.0,
            0.0,
            1.0,
            -1.0,
        );
        verts.extend(&vs);
        ids.extend(&is);

        let (vs, is) = colors(
            light_bgs[(frame / 150) % dark_bgs.len()],
            &light_colors,
            0.0,
            1.0,
            1.0,
            -1.0,
        );
        verts.extend(&vs);
        ids.extend(&is);

        let vbo = glium::VertexBuffer::new(&display, verts.as_slice()).unwrap();
        let ibo = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            ids.as_slice(),
        )
        .unwrap();

        let mut target = display.draw();
        let clear: [f32; 4] = Theme::LIGHT.bg.into();
        target.clear_color(clear[0], clear[1], clear[2], clear[3]);
        target
            .draw(
                &vbo,
                &ibo,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
