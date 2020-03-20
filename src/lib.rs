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
