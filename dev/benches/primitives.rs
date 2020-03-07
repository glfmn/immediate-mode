use criterion::{black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use immediate_mode::{color::theme, math::Vec2, DrawData};

fn draw_data() -> DrawData<Vert> {
    DrawData::<Vert> {
        verts: vec![],
        indicies: vec![],
    }
}

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

fn tri(c: &mut Criterion) {
    c.bench_function("tri", |b| {
        let mut draw = draw_data();
        b.iter(|| {
            draw.tri(
                theme::YELLOW,
                Vec2 { x: -1.0, y: 0.0 },
                Vec2 { x: 0.0, y: 0.0 },
                Vec2 { x: 0.0, y: -0.0 },
            )
        })
    });
}

fn tri_multicolor(c: &mut Criterion) {
    c.bench_function("tri_multicolor", |b| {
        let mut draw = draw_data();
        b.iter(|| {
            draw.tri_multicolor(
                (Vec2 { x: -1.0, y: 0.0 }, theme::YELLOW),
                (Vec2 { x: 1.0, y: 1.0 }, theme::RED),
                (Vec2 { x: 0.0, y: 1.0 }, theme::GREEN),
            )
        })
    });
}

criterion_group!(triangle, tri, tri_multicolor);

fn rect(c: &mut Criterion) {
    c.bench_function("rect", |b| {
        let mut draw = draw_data();
        b.iter(|| {
            draw.rect(
                theme::YELLOW,
                Vec2 { x: -1.0, y: 0.0 },
                Vec2 { x: 1.0, y: 1.0 },
            );
        })
    });
}

fn rect_uv(c: &mut Criterion) {
    c.bench_function("rect_uv", |b| {
        let mut draw = draw_data();
        b.iter(|| {
            draw.rect_uv(
                theme::YELLOW,
                (Vec2 { x: -1.0, y: 0.0 }, Vec2 { x: 0.0, y: 0.0 }),
                (Vec2 { x: 1.0, y: 1.0 }, Vec2 { x: 1.0, y: 1.0 }),
            )
        })
    });
}

criterion_group!(rectangle, rect, rect_uv);

fn polyline_sine(c: &mut Criterion) {
    macro_rules! test {
        ($polyline:ident) => {
            |b: &mut Bencher, n: &usize| {
                let mut xs = Vec::with_capacity(*n);
                for x in 0..*n {
                    let x = (x as f32) / (*n as f32);
                    let pos = Vec2 {
                        x: 2.0 * x - 1.0,
                        y: (x / 10.0).sin(),
                    };
                    xs.push(pos);
                }

                let mut draw = draw_data();
                b.iter(|| {
                    draw.$polyline(theme::RED, black_box(0.005), black_box(&xs));
                })
            }
        };
    }

    for points in &[8, 64, 512, 1024] {
        c.bench_with_input(
            BenchmarkId::new("polyline with n points", *points),
            points,
            test!(polyline),
        );
        c.bench_with_input(
            BenchmarkId::new("rect polyline with n points", *points),
            points,
            test!(rect_polyline),
        );
    }
}

fn polyline_2(c: &mut Criterion) {
    c.bench_function("2 point polyline", |b| {
        let mut draw = draw_data();
        b.iter(|| {
            draw.polyline(
                theme::RED,
                black_box(0.005),
                black_box(&[Vec2 { x: 0.0, y: 0.0 }, Vec2 { x: 0.1, y: 1.0 }]),
            );
        })
    });

    c.bench_function("2 point rect polyline", |b| {
        let mut draw = draw_data();
        b.iter(|| {
            draw.rect_polyline(
                theme::RED,
                black_box(0.005),
                black_box(&[Vec2 { x: 0.0, y: 0.0 }, Vec2 { x: 0.1, y: 1.0 }]),
            );
        })
    });
}

criterion_group!(polyline, polyline_sine, polyline_2,);

criterion_main!(triangle, rectangle, polyline);
