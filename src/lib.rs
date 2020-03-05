#![deny(missing_docs)]

//! # immediate-mode
//!
//! 2D immediate mode user interface for Rust

pub mod color;
pub mod math;

/// (position, uv, color)
pub(crate) type Vert = ([f32; 2], [f32; 2], [u8; 4]);

/// Data needed to draw the UI
///
/// ## Vertex
///
/// ```txt
/// (position, texture_coord, color): ([f32; 2], [f32; 2], [u8; 4])
/// ```
#[derive(Debug, Clone)]
pub struct DrawData<Vertex>
where
    Vertex: From<Vert> + Copy,
{
    /// Verticies
    pub verts: Vec<Vertex>,
    /// Index into each of the 3 vertex attribute arrays
    pub indicies: Vec<u32>,
}

impl<V> DrawData<V>
where
    V: From<Vert> + Copy,
{
    /// Triangle
    pub fn tri(&mut self, color: color::Color, a: math::Vec2, b: math::Vec2, c: math::Vec2) {
        let base_index = self.verts.len() as u32;

        let color: [u8; 4] = color.into();
        self.verts.extend(&[
            (a.into(), [0.0, 0.0], color).into(),
            (b.into(), [0.0, 0.0], color).into(),
            (c.into(), [0.0, 0.0], color).into(),
        ]);
        self.indicies
            .extend(&[base_index, base_index + 1, base_index + 2]);
    }

    /// Triangle with vertex colors set per-vertex
    pub fn tri_multicolor(
        &mut self,
        a: (math::Vec2, color::Color),
        b: (math::Vec2, color::Color),
        c: (math::Vec2, color::Color),
    ) {
        let base_index = self.verts.len() as u32;
        self.verts.extend(&[
            (a.0.into(), [0.0, 0.0], a.1.into()).into(),
            (b.0.into(), [0.0, 0.0], b.1.into()).into(),
            (c.0.into(), [0.0, 0.0], c.1.into()).into(),
        ]);
        self.indicies
            .extend(&[base_index, base_index + 1, base_index + 2]);
    }

    /// Add vertex data for a rectangle
    ///
    /// Rectangle is defined by the upper left and lower right coordinates
    pub fn rect(&mut self, color: color::Color, a: math::Vec2, b: math::Vec2) {
        let base_index = self.verts.len() as u32;

        let color: [u8; 4] = color.into();
        self.verts.extend(&[
            ([a.x, a.y], [0.0, 0.0], color).into(),
            ([a.x, b.y], [0.0, 0.0], color).into(),
            ([b.x, a.y], [0.0, 0.0], color).into(),
            ([b.x, b.y], [0.0, 0.0], color).into(),
        ]);
        self.indicies.extend(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index + 1,
            base_index + 2,
            base_index + 3,
        ]);
    }

    /// Add vertex data for a rectangle with specified UV coords
    ///
    /// Rectangle and its UVs defined by the upper left and lower right
    /// coordinates
    pub fn rect_uv(
        &mut self,
        color: color::Color,
        (a, uv_a): (math::Vec2, math::Vec2),
        (b, uv_b): (math::Vec2, math::Vec2),
    ) {
        let base_index = self.verts.len() as u32;

        let color: [u8; 4] = color.into();
        self.verts.extend(&[
            ([a.x, a.y], [uv_a.x, uv_a.y], color).into(),
            ([a.x, b.y], [uv_a.x, uv_b.y], color).into(),
            ([b.x, a.y], [uv_b.x, uv_a.y], color).into(),
            ([b.x, b.y], [uv_b.x, uv_b.y], color).into(),
        ]);
        self.indicies.extend(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index + 1,
            base_index + 2,
            base_index + 3,
        ])
    }

    /// A line drawn with polygons through the provided points
    pub fn polyline(&mut self, color: color::Color, thickness: f32, points: &[math::Vec2]) {
        // line must connect two points
        if points.len() < 2 {
            return;
        }

        let color: [u8; 4] = color.into();

        // create two verts per point to achieve thickness
        self.verts.reserve(2 * points.len());
        // use two triangles per segment that connects two points
        // n points, n-1 segments
        self.indicies.reserve((points.len() - 1) * 6);

        // Place the first points perpendicular to the line segment
        let df = points[0] - points[1];
        let nf = df.normal().unit() * thickness;

        let index_count = self.verts.len() as u32;
        self.verts.extend(&[
            ((points[0] - nf).into(), [0.0, 0.0], color).into(),
            ((points[0] + nf).into(), [0.0, 0.0], color).into(),
        ]);
        // push indicies joining this point to the next point's segment
        self.indicies.extend(&[
            index_count,
            index_count + 1,
            index_count + 2,
            index_count + 1,
            index_count + 2,
            index_count + 3,
        ]);

        // iterate over pairs of indicies
        for i1 in 1..(points.len() - 1) {
            let p0 = points[i1 - 1];
            let p1 = points[i1];
            let p2 = points[i1 + 1];

            // calculate the direction of the line going into the point and its normal
            let d_in = p0 - p1;
            let n01 = d_in.unit().normal();

            // calculate the tangent of join between lines and get its normal
            let miter = ((p2 - p1).unit() + (p1 - p0).unit()).unit().normal();

            // project the miter line onto the normal and use it to calculate the
            // length of the miter line needed to join the line segments
            let length = thickness / miter.dot(n01);

            // push verticies and indicies joining this point to the _next_ point
            let index_count = self.verts.len() as u32;
            self.verts.extend(&[
                ((p1 - miter * length).into(), [0.0, 0.0], color).into(),
                ((p1 + miter * length).into(), [0.0, 0.0], color).into(),
            ]);
            self.indicies.extend(&[
                index_count,
                index_count + 1,
                index_count + 2,
                index_count + 1,
                index_count + 2,
                index_count + 3,
            ]);
        }

        // Place the last points perpendicular to the line segment
        let last = points.len() - 1;
        let dl = points[last - 1] - points[last];
        let nl = dl.normal().unit() * thickness;

        self.verts.extend(&[
            ((points[last] - nl).into(), [0.0, 0.0], color).into(),
            ((points[last] + nl).into(), [0.0, 0.0], color).into(),
        ]);
    }
}
