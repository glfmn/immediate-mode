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

use std::collections::*;

fn load_font(font_data: &[u8]) -> (immediate_mode::text::Texture, HashMap<char, (Vec2, Vec2)>) {
    use immediate_mode::text::{point, Font, Scale};

    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(font_data).expect("Error constructing Font");
    // The font size to use
    let scale = Scale::uniform(20.0 * 2.0);
    let v_metrics = font.v_metrics(scale);
    let ascent = v_metrics.ascent as u32;

    let mut texture = Texture::new(1024, 1024);

    for y in 0..(v_metrics.ascent - v_metrics.descent).ceil() as u32 {
        texture[(0, y)] = 255;
    }

    let mut cursor: (i32, i32) = (4, ascent as i32);
    let mut atlas = HashMap::with_capacity(128 - 32);

    // Loop through the glyphs in the text, positing each one on a line
    for c in (32u8..128)
        .map(|c| c as char)
        .chain("žБВГИІЇЙЈКЛЉМНЊОПРСТЋУЎ".chars())
    {
        let glyph = font.glyph(c).scaled(scale).positioned(point(0.0, 0.0));
        if let Some(bb) = glyph.pixel_bounding_box() {
            let y_min = bb.min.y;
            let x_min = bb.min.x;
            glyph.draw(|x, y, v| {
                texture[(x + (cursor.0 + x_min) as u32, y + (cursor.1 + y_min) as u32)] =
                    (v * 255.0) as u8;
            });
            atlas.insert(
                c,
                (
                    Vec2::new(
                        (cursor.0 + bb.min.x) as f32 / 1024.0,
                        (cursor.1 + bb.min.y) as f32 / 1024.0,
                    ),
                    Vec2::new(
                        (cursor.0 + bb.max.x) as f32 / 1024.0,
                        (cursor.1 + bb.max.y) as f32 / 1024.0,
                    ),
                ),
            );
            cursor.0 += bb.max.x + 1;
            if cursor.0 > 1024 - 16 {
                cursor.0 = 0;
                cursor.1 += (v_metrics.ascent - v_metrics.descent).ceil() as i32;
            }
        }
    }

    // display font
    // for uvs in atlas.values() {
    // draw.rect_uv(
    // Theme::DARK.fg_disabled,
    // (uvs.0 * 1024.0, uvs.0),
    // (uvs.1 * 1024.0, uvs.1),
    // );
    // }
    (texture, atlas)
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("immediate-mode");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut frame: usize = 0;

    let program =
        glium::Program::from_source(&display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap();

    let mut cursor_pos = glutin::dpi::PhysicalPosition::new(0.0, 0.0);
    let mut cursor_down = false;
    let mut ui: UI<Vert> = UI::new(Input::new(None, false));
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

        ui.next_frame(Input::new(
            Some(Vec2::new(cursor_pos.x as f32, cursor_pos.y as f32)),
            cursor_down,
        ));

        frame += 1;

        let (width, height) = display.get_framebuffer_dimensions();
        let scale_factor = display.gl_window().window().scale_factor();

        ui.draw(|data| {
            data.rect(
                Theme::DARK.bg,
                Vec2::new(0.0, 0.0),
                Vec2::new(width as f32, height as f32),
            )
        });

        use immediate_mode as im;

        fn draw_button<V>(ui: &mut UI<V>, color: Color, pos: Vec2) -> (Vec2, Vec2)
        where
            V: From<im::draw::Vert> + Copy,
        {
            let size = Vec2 { x: 100.0, y: 20.0 };
            ui.draw(|data| {
                data.rect(color, pos, pos + size);
            });

            (pos, pos + size)
        }

        fn button<S: AsRef<str>, V>(ui: &mut UI<V>, label: &S, pos: Vec2) -> im::Event
        where
            V: From<im::draw::Vert> + Copy,
        {
            let id = ui.calculate_id(label.as_ref());

            let color = if ui.is_held(id) {
                Theme::DARK.active
            } else {
                if ui.is_hovered(id) {
                    Theme::DARK.hover
                } else {
                    Theme::DARK.element
                }
            };

            let region = draw_button(ui, color, pos);
            ui.event(id, region)
        }

        ui.with_id(ui.calculate_id("SCOPE"), |ui| {
            button(ui, &"Hello", Vec2::new(10.0, 10.0))
                .on_hover(|_| println!("{:#x} HOVERED 1", frame))
                .on_hold(|_| println!("{:#x} HELD    1", frame))
                .on_click(|_| println!("{:#x} CLICKED 1", frame))
                .tooltip(ui, &"Hello");
        });

        button(&mut ui, &"Hello", Vec2::new(10.0, 100.0))
            .on_hover(|_| println!("{:#x} HOVERED 2", frame))
            .on_hold(|_| println!("{:#x} HELD    2", frame))
            .on_click(|_| println!("{:#x} CLICKED 2", frame));

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
    });
}
