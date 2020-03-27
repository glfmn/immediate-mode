//! Loading fonts to the GPU for immediate-mode to use

use std::ops::{Index, IndexMut};

pub use rusttype::*;

/// 2D grayscale texture
#[derive(Debug, Clone)]
pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Index<(u32, u32)> for Texture {
    type Output = u8;

    /// Get pixel at (x,y)
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        &self.pixels[y as usize * self.width + x as usize]
    }
}

impl IndexMut<(u32, u32)> for Texture {
    /// Get mutable access to pixel at (x,y)
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        &mut self.pixels[y as usize * self.width + x as usize]
    }
}

impl Texture {
    /// Create a texture of the desired width and height
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0u8; width * height];
        Texture {
            width,
            height,
            pixels,
        }
    }

    /// Width and height of the texture
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    /// Pixels as value only
    ///
    /// row-by-row
    pub fn pixels(&self) -> &[u8] {
        self.pixels.as_slice()
    }
}
