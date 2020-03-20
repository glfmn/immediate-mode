#![deny(missing_docs)]

//! # immediate-mode
//!
//! 2D immediate mode user interface for Rust

mod color;
pub mod draw;
mod math;
mod text;

pub use crate::color::{theme, Color, Theme};
pub use crate::math::Vec2;
