#![deny(missing_docs)]

//! # immediate-mode
//!
//! 2D immediate mode user interface for Rust

pub mod draw;
pub mod text;

// modules for code organization:

mod color;
mod math;

pub use crate::color::{theme, Color, Theme};
pub use crate::math::Vec2;

use std::collections::hash_map::RandomState;
use std::fmt::{self, Debug, Formatter};

/// High level input consumed by the UI
#[derive(Debug, Clone)]
pub struct Input {
    mouse_pos: (u32, u32),
    mouse_down: bool,
}

impl Input {
    /// Create input necessary to process the UI
    pub fn new((mouse_x, mouse_y): (u32, u32), mouse_down: bool) -> Self {
        Input {
            mouse_pos: (mouse_x, mouse_y),
            mouse_down,
        }
    }
}

/// Interface used to gather commands which draw a single frame of the UI
///
/// When updating finishes, call `finish_frame` to expose rendering data
#[derive(Debug, Clone)]
pub struct UI<V>
where
    V: From<draw::Vert> + Copy,
{
    draw_data: draw::DrawData<V>,
    context: Context,
    id_stack: Vec<u64>,
    input: Input,
}

impl<V> UI<V>
where
    V: From<draw::Vert> + Copy,
{
    /// Create the first_frame UI
    pub fn new(input: Input) -> Self {
        use crate::draw::DrawData;

        UI {
            id_stack: Vec::with_capacity(8),
            input: input,
            context: Context::default(),
            draw_data: DrawData::<V>::default(),
        }
    }

    /// Draw primitives directly to the draw data
    pub fn draw<F>(&mut self, command: F)
    where
        F: FnOnce(&mut draw::DrawData<V>),
    {
        command(&mut self.draw_data)
    }

    /// Derive an ID to keep track of an element between frames
    pub fn calculate_id<H: std::hash::Hash>(&self, into_id: H) -> ID {
        use std::hash::{BuildHasher, Hash, Hasher};
        let mut hasher = self.context.id_hasher.build_hasher();

        self.id_stack.last().hash(&mut hasher);
        into_id.hash(&mut hasher);

        hasher.finish()
    }

    /// Add an ID to the stack from which all other IDs will derive
    pub fn with_id<F: FnOnce(&mut Self)>(&mut self, id: ID, exec: F) -> &mut Self {
        self.id_stack.push(id);
        exec(self);
        self.id_stack.pop();
        self
    }

    /// Complete this frame of the UI and render
    pub fn finish_frame<'a>(&'a mut self) -> Renderer<'a, V> {
        Renderer { ui: self }
    }

    /// Was this ID previously declared active?
    ///
    /// By convention, activeness is always set after it is checked so this
    /// means it must have been set active during a _previous_ frame.
    pub fn is_active(&self, id: ID) -> bool {
        id == self.context.active_id
    }

    /// Check a region associated with an ID for user interaction
    pub fn event(&self, id: ID, region: (Vec2, Vec2)) -> Event {
        let (x, y) = self.input.mouse_pos;
        let (x, y) = (x as f32, y as f32);

        let hit = region.0.x < x && x < region.1.x && region.0.y < y && y < region.1.y;
        let active = id == self.context.active_id;
        let hot = active && hit;

        Event {
            is_clicked: self.input.mouse_down && hot,
            is_hovered: !self.input.mouse_down && hot,
            mouse_pos: Vec2 { x, y },
        }
    }

    /// This element ID is the active one for the current frame
    pub fn set_active(&mut self, id: ID) {
        self.context.active_id = id;
    }
}

/// Unique identifier for a UI element
pub type ID = u64;

/// User-Interface data which must persist between frames
#[derive(Clone)]
pub(crate) struct Context {
    active_id: ID,
    id_hasher: RandomState,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            active_id: 0,
            id_hasher: RandomState::new(),
        }
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            // show hex for active_id since it is a hash
            .field("active_id", &format!("{:#x}", self.active_id))
            .finish()
    }
}

/// Result of a user interaction with a specific region of the UI
#[derive(Debug)]
pub struct Event {
    is_clicked: bool,
    is_hovered: bool,
    mouse_pos: Vec2,
}

impl Event {
    /// Perform an action when the UI detects a click
    pub fn on_click<F: FnOnce()>(&self, action: F) -> &Self {
        if self.is_clicked {
            action();
        }
        self
    }

    /// Perform an action when hovering over the UI
    pub fn on_hover<F: FnOnce(Vec2)>(&self, action: F) -> &Self {
        if self.is_hovered && !self.is_clicked {
            action(self.mouse_pos);
        }
        self
    }
}

/// Interface necessary to access data for rendering a frame
///
/// Once rendering is done, call `next_frame` to get the UI
#[derive(Debug)]
pub struct Renderer<'a, V>
where
    V: From<draw::Vert> + Copy,
{
    ui: &'a mut UI<V>,
}

impl<'a, V> Renderer<'a, V>
where
    V: From<draw::Vert> + Copy,
{
    /// Process UI for the next frame
    pub fn next_frame(self, input: Input) -> &'a mut UI<V> {
        self.ui.input = input;
        self.ui
    }

    /// Access the verticies produced by the renderer
    pub fn verts(&self) -> &[V] {
        self.ui.draw_data.verts()
    }

    /// Access the indicies produced by the renderer
    pub fn indicies(&self) -> &[u32] {
        self.ui.draw_data.indicies()
    }
}
