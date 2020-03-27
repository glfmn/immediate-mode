//! Low-level interface for drawing UI primitives

use crate::color::Color;
use crate::Vec2;

/// Vertex data is always in the format (position, uv, rgba)
///
/// Define a conversion from `Vert` to your own vertex type to
/// integrate smoothly with your renderer.
pub type Vert = ([f32; 2], [f32; 2], [u8; 4]);

/// Texture coordinate for drawing fully opaque primitives
///
/// For example, using this coordinate for every vertex in a triangle
/// guarantees an opque triangle
pub const OPAQUE_UV: [f32; 2] = [0.0, 0.0];

/// Data needed to draw the UI
#[derive(Debug, Clone)]
pub struct DrawData<Vertex>
where
    Vertex: From<Vert> + Copy,
{
    /// Verticies
    pub(crate) verts: Vec<Vertex>,
    /// Index into each of the 3 vertex attribute arrays
    pub(crate) indicies: Vec<u32>,
}

impl<Vertex> Default for DrawData<Vertex>
where
    Vertex: From<Vert> + Copy,
{
    fn default() -> Self {
        DrawData {
            verts: Vec::with_capacity(32),
            indicies: Vec::with_capacity(64),
        }
    }
}

/// Very common index pattern used when pushing indicies for a quad of verts
macro_rules! quad_indicies {
    ($first_index:expr) => {
        [
            // first triangle
            $first_index,
            $first_index + 1,
            $first_index + 2,
            // second triangle
            $first_index + 1,
            $first_index + 2,
            $first_index + 3,
        ]
    };
}

impl<V> DrawData<V>
where
    V: From<Vert> + Copy,
{
    /// Add verts directly to the buffer with relative indicies
    ///
    /// Indicies are relative to the verts passed in; for example:
    /// ```
    /// use immediate_mode::draw::{DrawData, OPAQUE_UV};
    ///
    /// type Vert = ([f32; 2], [f32; 2], [u8; 4]);
    /// let mut draw_data = DrawData::<Vert>::default();
    ///
    /// // add a triangle:
    /// let verts = &[
    ///     (OPAQUE_UV, OPAQUE_UV, [1,0,0,1]),
    ///     ([0.0, 0.5], [1.0, 0.0], [0,1,0,1]),
    ///     ([0.5, 0.0], [0.0, 1.0], [0,0,1,1]),
    /// ];
    /// let indicies = &[0, 1, 2];
    /// draw_data.extend(verts, indicies);
    ///
    /// // add a second triangle
    /// let verts = &[
    ///     ([1.0, 0.0], OPAQUE_UV, [1,0,1,1]),
    ///     ([1.0, 0.5], [1.0, 0.0], [1,1,0,1]),
    ///     ([0.5, 1.0], [0.0, 1.0], [0,1,1,1]),
    /// ];
    /// let indicies = &[0, 1, 2];
    /// draw_data.extend(verts, indicies);
    /// ```
    #[inline]
    pub fn extend(&mut self, verts: &[V], indicies: &[u32]) {
        let base_index = self.verts.len() as u32;
        self.verts.extend(verts);
        self.indicies
            .extend(indicies.iter().map(|i| base_index + i));
    }

    /// Retrieve verticies
    #[inline(always)]
    pub fn verts(&self) -> &[V] {
        self.verts.as_slice()
    }

    /// Retrieve indicies
    #[inline(always)]
    pub fn indicies(&self) -> &[u32] {
        self.indicies.as_slice()
    }

    /// Triangle with uniform color
    pub fn tri(&mut self, color: Color, a: Vec2, b: Vec2, c: Vec2) {
        let base_index = self.verts.len() as u32;

        let color: [u8; 4] = color.into();
        self.verts.extend(&[
            (a.into(), OPAQUE_UV, color).into(),
            (b.into(), OPAQUE_UV, color).into(),
            (c.into(), OPAQUE_UV, color).into(),
        ]);
        self.indicies
            .extend(&[base_index, base_index + 1, base_index + 2]);
    }

    /// Triangle with vertex colors set per-vertex
    pub fn tri_multicolor(&mut self, a: (Vec2, Color), b: (Vec2, Color), c: (Vec2, Color)) {
        let base_index = self.verts.len() as u32;
        self.verts.extend(&[
            (a.0.into(), OPAQUE_UV, a.1.into()).into(),
            (b.0.into(), OPAQUE_UV, b.1.into()).into(),
            (c.0.into(), OPAQUE_UV, c.1.into()).into(),
        ]);
        self.indicies
            .extend(&[base_index, base_index + 1, base_index + 2]);
    }

    /// Add vertex data for a rectangle
    ///
    /// Rectangle is defined by the upper left and lower right coordinates
    /// which means it is always axis aligned to the screen coordinates.
    pub fn rect(&mut self, color: Color, a: Vec2, b: Vec2) {
        let base_index = self.verts.len() as u32;

        let color: [u8; 4] = color.into();
        self.verts.extend(&[
            ([a.x, a.y], OPAQUE_UV, color).into(),
            ([a.x, b.y], OPAQUE_UV, color).into(),
            ([b.x, a.y], OPAQUE_UV, color).into(),
            ([b.x, b.y], OPAQUE_UV, color).into(),
        ]);
        self.indicies.extend(&quad_indicies![base_index]);
    }

    /// Add vertex data for a rectangle with specified UV coords
    ///
    /// Rectangle and its UVs defined by the upper left and lower right
    /// coordinates which means they are always axis aligned to the
    /// screen coordinates.
    pub fn rect_uv(&mut self, color: Color, (a, uv_a): (Vec2, Vec2), (b, uv_b): (Vec2, Vec2)) {
        let base_index = self.verts.len() as u32;

        let color: [u8; 4] = color.into();
        self.verts.extend(&[
            ([a.x, a.y], [uv_a.x, uv_a.y], color).into(),
            ([a.x, b.y], [uv_a.x, uv_b.y], color).into(),
            ([b.x, a.y], [uv_b.x, uv_a.y], color).into(),
            ([b.x, b.y], [uv_b.x, uv_b.y], color).into(),
        ]);
        self.indicies.extend(&quad_indicies![base_index])
    }

    /// Draw a line with polygons
    ///
    /// The line will have two verticies per point on the miter line, that is,
    /// the verticies are aligned to the join between segments so it looks like
    /// they cleanly join.
    ///
    /// This means that only 2 verts per point are generated, but the position
    /// for each vert requires more math to compute compared to `rect_polyline`
    /// ```
    /// use immediate_mode::{ draw::DrawData, Color, Vec2 };
    ///
    /// # type Vert = ([f32; 2], [f32; 2], [u8; 4]);
    /// let mut draw_data = DrawData::<Vert>::default();
    ///
    /// // draw 3 points
    /// let points = &[
    ///     Vec2 { x: 0.0, y: 1.0 },
    ///     Vec2 { x: 0.5, y: 0.5 },
    ///     Vec2 { x: 1.0, y: 0.0 },
    /// ];
    /// draw_data.polyline(Color(0xFF_FF_FF_FF), 1.0, points);
    ///
    /// assert_eq!(points.len() * 2, draw_data.verts().len());
    /// assert_eq!((points.len()-1) * 6, draw_data.indicies().len());
    /// ```
    pub fn polyline(&mut self, color: Color, thickness: f32, points: &[Vec2]) {
        // line must connect two points
        if points.len() < 2 {
            return;
        }

        let color: [u8; 4] = color.into();
        let thickness = thickness * 0.5;

        // Draw the line with two vertices per point.  The verts are placed
        // on the miter line.  This line is essentially the intersection of
        // the rectangles which form the segments on the line, forming a corner

        self.verts.reserve(2 * points.len()); // 2 verts per point
        self.indicies.reserve((points.len() - 1) * 6); // 2 tris per segment

        // Place the first points perpendicular to the line segment from
        // the first to second point
        let df = points[0] - points[1];
        let nf = df.normal().unit() * thickness;
        let first_index = self.verts.len() as u32;
        self.verts.extend(&[
            ((points[0] + nf).into(), OPAQUE_UV, color).into(),
            ((points[0] - nf).into(), OPAQUE_UV, color).into(),
        ]);
        // push indicies joining this point to the next point's verts
        self.indicies.extend(&quad_indicies![first_index]);

        // iterate over pairs of indicies
        for i1 in 1..(points.len() - 1) {
            let p0 = points[i1 - 1];
            let p1 = points[i1];
            let p2 = points[i1 + 1];

            // calculate the direction of the line going into the point and its normal
            let d_in = p1 - p0;
            let n01 = d_in.normal().unit();

            // calculate the tangent of join between lines and get its normal
            let miter = ((p2 - p1).unit() + (p1 - p0).unit()).unit().normal();

            // project the miter line onto the normal and use it to calculate the
            // length of the miter line needed to join the line segments
            let length = thickness / miter.dot(n01);

            // push indicies joining this point to the _next_ point
            // but only push the verticies for this point along the miter line
            let first_index = self.verts.len() as u32;
            self.verts.extend(&[
                ((p1 - miter * length).into(), OPAQUE_UV, color).into(),
                ((p1 + miter * length).into(), OPAQUE_UV, color).into(),
            ]);
            self.indicies.extend(&quad_indicies![first_index]);
        }

        // Place the last points perpendicular to the line segment as with the
        // first points, indicies have already been pushed on
        let last = points.len() - 1;
        let dl = points[last] - points[last - 1];
        let nl = dl.normal().unit() * thickness;
        self.verts.extend(&[
            ((points[last] - nl).into(), OPAQUE_UV, color).into(),
            ((points[last] + nl).into(), OPAQUE_UV, color).into(),
        ]);
    }

    /// Generates a line from rectangles
    ///
    /// Sometimes faster alternative to `polyline`; rather than joining line
    /// segments at the miter line, draw a rectangle aligned to the segment.
    ///
    /// This generates `(points.len()-1)*4` verticies but requires less math
    /// than `polyline` and so can be faster.
    ///
    /// ```
    /// use immediate_mode::{ draw::DrawData, Color, Vec2 };
    ///
    /// # type Vert = ([f32; 2], [f32; 2], [u8; 4]);
    /// let mut draw_data = DrawData::<Vert>::default();
    ///
    /// // draw 3 points
    /// let points = &[
    ///     Vec2 { x: 0.0, y: 1.0 },
    ///     Vec2 { x: 0.5, y: 0.5 },
    ///     Vec2 { x: 1.0, y: 0.0 },
    /// ];
    /// draw_data.rect_polyline(Color(0xFF_FF_FF_FF), 1.0, points);
    ///
    /// assert_eq!((points.len()-1) * 4, draw_data.verts().len());
    /// assert_eq!((points.len()-1) * 6, draw_data.indicies().len());
    /// ```
    pub fn rect_polyline(&mut self, color: Color, thickness: f32, points: &[Vec2]) {
        // line must connect two points
        if points.len() < 2 {
            return;
        }

        let color: [u8; 4] = color.into();
        let thickness = thickness * 0.5;

        // Draw a rectangle for each segment which joins two points with no
        // joining between the two segments
        self.verts.reserve(4 * (points.len() - 1)); // 4 verts per segment
        self.indicies.reserve((points.len() - 1) * 6); // 2 tris per segment

        // Place a rectangle along the first line segment
        let df = points[0] - points[1];
        let nf = df.normal().unit() * thickness;
        let first_index = self.verts.len() as u32;
        self.verts.extend(&[
            ((points[0] - nf).into(), OPAQUE_UV, color).into(),
            ((points[0] + nf).into(), OPAQUE_UV, color).into(),
            ((points[1] - nf).into(), OPAQUE_UV, color).into(),
            ((points[1] + nf).into(), OPAQUE_UV, color).into(),
        ]);
        self.indicies.extend(&quad_indicies![first_index]);

        // iterate over pairs of indicies, or segments, and draw a rectangle
        // for each
        for i1 in 1..(points.len() - 1) {
            let p1 = points[i1];
            let p2 = points[i1 + 1];

            // calculate the direction of the line going along this segment
            // and draw the rectangle for the segment
            let d_in = p2 - p1;
            let n = d_in.normal().unit();
            let first_index = self.verts.len() as u32;
            self.verts.extend(&[
                ((p1 - n * thickness).into(), OPAQUE_UV, color).into(),
                ((p1 + n * thickness).into(), OPAQUE_UV, color).into(),
                ((p2 - n * thickness).into(), OPAQUE_UV, color).into(),
                ((p2 + n * thickness).into(), OPAQUE_UV, color).into(),
            ]);
            self.indicies.extend(&quad_indicies![first_index]);
        }
    }
}
