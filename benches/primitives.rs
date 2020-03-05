use criterion::{black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use immediate_mode::{color::theme, math::Vec2, DrawData};

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

fn sine(c: &mut Criterion) {
    let n_xs = |b: &mut Bencher, n: &usize| {
        let mut xs = Vec::with_capacity(*n);
        for x in 0..*n {
            let x = (x as f32) / (*n as f32);
            let pos = Vec2 {
                x: 2.0 * x - 1.0,
                y: (x / 10.0).sin(),
            };
            xs.push(pos);
        }

        let mut draw = DrawData::<Vert> {
            verts: vec![],
            indicies: vec![],
        };
        b.iter(|| {
            draw.polyline(theme::RED, black_box(0.005), black_box(&xs));
        })
    };

    let mut points = 1;
    for _ in 0..10 {
        points *= 2;
        c.bench_with_input(
            BenchmarkId::new("polyline with n points", points),
            &points,
            n_xs,
        );
    }
}

criterion_group!(polyline, sine);
criterion_main!(polyline);
