//! 2D arithmetic types for immediate-mode

use std::ops::{Add, Mul, Sub};

/// 2D math vector
///
/// Defining `Into` and `From` for your math types allows seamless
/// integration with all public API functions that require 2D positions as
/// input.
///
/// ```
/// use immediate_mode as im;
/// struct Vec2([f32; 2]);
///
/// impl Into<im::Vec2> for Vec2 {
///     fn into(self) -> im::Vec2 {
///         self.0.into()
///     }
/// }
/// ```
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec2 {
    /// X-dimension horizontal component; zero at the left edge of the screen
    pub x: f32,
    /// Y-dimension vertical component; zero at the top edge of the screen
    pub y: f32,
}

impl Vec2 {
    /// A zero vector
    ///
    /// ```
    /// use immediate_mode::Vec2;
    ///
    /// assert_eq!(Vec2::zero(), Vec2::default());
    /// ```
    #[inline(always)]
    pub const fn zero() -> Self {
        Vec2 { x: 0f32, y: 0f32 }
    }

    /// Apply a function to the vector's x and y
    #[inline(always)]
    pub(crate) fn map<F: Fn(f32) -> f32>(self, f: F) -> Self {
        Vec2 {
            x: f(self.x),
            y: f(self.y),
        }
    }

    /// Dot product between two vectors
    #[inline(always)]
    pub(crate) fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Normal of a vector
    #[inline(always)]
    pub(crate) fn normal(self) -> Self {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }

    /// Sqaured magnitude of a vector
    #[inline(always)]
    pub(crate) fn len2(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Magnitude of a vector
    #[inline(always)]
    pub(crate) fn len(self) -> f32 {
        self.len2().sqrt()
    }

    /// Vector with a magnitude of 1
    #[inline(always)]
    pub(crate) fn unit(self) -> Self {
        self * (1.0 / self.len().max(0.000_000_01))
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::zero()
    }
}

// Math operators

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f32) -> Self::Output {
        self.map(|n| n + rhs)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f32) -> Self::Output {
        self.map(|n| n - rhs)
    }
}

impl Mul for Vec2 {
    type Output = f32;

    /// The vector dot product
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2::dot(self, rhs)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    /// The scalar product
    fn mul(self, rhs: f32) -> Self::Output {
        self.map(|n| n * rhs)
    }
}

// Conversion to and from primitive types

impl Into<(f32, f32)> for Vec2 {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Vec2 { x, y }
    }
}

impl Into<[f32; 2]> for Vec2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(v: [f32; 2]) -> Self {
        Vec2 { x: v[0], y: v[1] }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn scalar_product() {
        use super::Vec2;

        let a = Vec2 { x: 1.0, y: 1.0 };
        let b: (f32, f32) = (a * 2.0).into();
        assert_eq!(b, (2.0, 2.0));
    }
}
