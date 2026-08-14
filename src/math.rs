//! Math helpers for the animation engine.
//!
//! Provides a 2D vector with the arithmetic operators and normalization
//! helpers the physics engine needs, plus an axis-aligned `Rect` used for
//! collision bounds and a `Size` for collision dimensions.

use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

// ═══════════════════════════════════════════════════════════════
// Vec2
// ═══════════════════════════════════════════════════════════════

/// A 2D vector.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Euclidean length of the vector.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Unit-length vector in the same direction. Returns [`Vec2::ZERO`]
    /// when the length is (near) zero.
    pub fn normalized(&self) -> Vec2 {
        let len = self.length();
        if len <= f32::EPSILON {
            Vec2::ZERO
        } else {
            Vec2::new(self.x / len, self.y / len)
        }
    }

    /// Dot product with another vector.
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Scale both components by `scalar`.
    pub fn scale(&self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

/// Convenience constructor: `v(10.0, 20.0)`.
pub fn v(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

// ═══════════════════════════════════════════════════════════════
// Size
// ═══════════════════════════════════════════════════════════════

/// A 2D size used for collision bounds of physics items.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}

// ═══════════════════════════════════════════════════════════════
// Rect
// ═══════════════════════════════════════════════════════════════

/// An axis-aligned rectangle in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Minimum corner (top-left).
    pub fn min(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Maximum corner (bottom-right).
    pub fn max(&self) -> Vec2 {
        Vec2::new(self.x + self.width, self.y + self.height)
    }

    /// Whether `p` lies on or inside the rectangle.
    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.x
            && p.y >= self.y
            && p.x <= self.x + self.width
            && p.y <= self.y + self.height
    }

    /// Clamp `p` to the rectangle (inclusive).
    pub fn clamp(&self, p: Vec2) -> Vec2 {
        Vec2::new(
            p.x.clamp(self.x, self.x + self.width),
            p.y.clamp(self.y, self.y + self.height),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_ops() {
        assert_eq!(v(1.0, 2.0) + v(3.0, 4.0), v(4.0, 6.0));
        assert_eq!(v(3.0, 4.0).length(), 5.0);
        assert_eq!(v(2.0, 0.0).normalized(), v(1.0, 0.0));
        assert_eq!(v(0.0, 0.0).normalized(), v(0.0, 0.0));
    }

    #[test]
    fn rect_contains() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(r.contains(v(50.0, 50.0)));
        assert!(!r.contains(v(150.0, 50.0)));
        assert_eq!(r.clamp(v(200.0, 200.0)), v(100.0, 100.0));
    }
}