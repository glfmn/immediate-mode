#![deny(missing_docs)]

//! # immediate-mode
//!
//! 2D immediate mode user interface for Rust

mod color;
mod math;

pub use crate::color::{theme, Color, Theme};
pub use crate::math::Vec2;

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

/// Very common index pattern used when pushing indicies for a quad of verts
macro_rules! quad_indicies {
    ($index_count:expr) => {
        [
            // first triangle
            $index_count,
            $index_count + 1,
            $index_count + 2,
            // second triangle
            $index_count + 1,
            $index_count + 2,
            $index_count + 3,
        ]
    };
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
        self.indicies.extend(&quad_indicies![base_index]);
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
        self.indicies.extend(&quad_indicies![base_index])
    }

    /// A line drawn with polygons through the provided points
    pub fn polyline(&mut self, color: color::Color, thickness: f32, points: &[math::Vec2]) {
        // line must connect two points
        if points.len() < 2 {
            return;
        }

        let color: [u8; 4] = color.into();
        let thickness = thickness * 0.5;

        // Draw the line with two vertices per point.  The verts are placed
        // on the miter line.  This line is essentially the intersection of
        // the rectangles which form the segments on the line, forming a corner

        self.verts.reserve(2 * points.len());
        self.indicies.reserve((points.len() - 1) * 6); // 2 tris per segment

        // Place the first points perpendicular to the line segment from
        // the first to second point
        let df = points[0] - points[1];
        let nf = df.normal().unit() * thickness;
        let index_count = self.verts.len() as u32;
        self.verts.extend(&[
            ((points[0] + nf).into(), [0.0, 0.0], color).into(),
            ((points[0] - nf).into(), [0.0, 0.0], color).into(),
        ]);
        // push indicies joining this point to the next point's verts
        self.indicies.extend(&quad_indicies![index_count]);

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
            let index_count = self.verts.len() as u32;
            self.verts.extend(&[
                ((p1 - miter * length).into(), [0.0, 0.0], color).into(),
                ((p1 + miter * length).into(), [0.0, 0.0], color).into(),
            ]);
            self.indicies.extend(&quad_indicies![index_count]);
        }

        // Place the last points perpendicular to the line segment as with the
        // first points, indicies have already been pushed on
        let last = points.len() - 1;
        let dl = points[last] - points[last - 1];
        let nl = dl.normal().unit() * thickness;
        self.verts.extend(&[
            ((points[last] - nl).into(), [0.0, 0.0], color).into(),
            ((points[last] + nl).into(), [0.0, 0.0], color).into(),
        ]);
    }

    /// fast polyline
    pub fn rect_polyline(&mut self, color: color::Color, thickness: f32, points: &[math::Vec2]) {
        // line must connect two points
        if points.len() < 2 {
            return;
        }

        let color: [u8; 4] = color.into();
        let thickness = thickness * 0.5;

        // Draw a rectangle for each segment which joins two points with no
        // joining between the two segments
        self.verts.reserve(4 * points.len());
        self.indicies.reserve((points.len() - 1) * 6); // 2 tris per segment

        // Place a rectangle along the first line segment
        let df = points[0] - points[1];
        let nf = df.normal().unit() * thickness;
        let index_count = self.verts.len() as u32;
        self.verts.extend(&[
            ((points[0] - nf).into(), [0.0, 0.0], color).into(),
            ((points[0] + nf).into(), [0.0, 0.0], color).into(),
            ((points[1] - nf).into(), [0.0, 0.0], color).into(),
            ((points[1] + nf).into(), [0.0, 0.0], color).into(),
        ]);
        self.indicies.extend(&quad_indicies![index_count]);

        // iterate over pairs of indicies, or segments, and draw a rectangle
        // for each
        for i1 in 1..(points.len() - 1) {
            let p1 = points[i1];
            let p2 = points[i1 + 1];

            // calculate the direction of the line going along this segment
            // and draw the rectangle for the segment
            let d_in = p2 - p1;
            let n = d_in.normal().unit();
            let index_count = self.verts.len() as u32;
            self.verts.extend(&[
                ((p1 - n * thickness).into(), [0.0, 0.0], color).into(),
                ((p1 + n * thickness).into(), [0.0, 0.0], color).into(),
                ((p2 - n * thickness).into(), [0.0, 0.0], color).into(),
                ((p2 + n * thickness).into(), [0.0, 0.0], color).into(),
            ]);
            self.indicies.extend(&quad_indicies![index_count]);
        }
    }
}
