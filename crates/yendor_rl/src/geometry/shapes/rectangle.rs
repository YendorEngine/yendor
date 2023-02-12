#![allow(dead_code)]

use crate::prelude::*;

/// A 2D rectangle.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rectangle {
    /// The minimum coordinates of the rectangle.
    pub min: IVec2,
    /// The maximum coordinates of the rectangle.
    pub max: IVec2,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new_with_size(IVec2::ZERO, UVec2::ONE)
    }
}

impl Rectangle {
    /// Creates a new rectangle.
    #[inline]
    pub fn new(min: IVec2, max: IVec2) -> Self {
        Self {
            min: min.min(max),
            max: min.max(max),
        }
    }

    /// Creates a new rectangle with the given size.
    #[inline]
    pub fn new_with_size(min: IVec2, dimensions: UVec2) -> Self {
        Self::new(min, min + dimensions.as_ivec2())
    }
}

impl Rectangle {
    /// Get the width of the rectangle.
    #[inline]
    pub const fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    /// Get the height of the rectangle.
    #[inline]
    pub const fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    /// Get the minimum point of the rectangle.
    #[inline]
    pub const fn min(&self) -> IVec2 {
        self.min
    }

    /// Get the maximum point of the rectangle.
    #[inline]
    pub const fn max(&self) -> IVec2 {
        self.max
    }

    /// Check if the rectangle is square.
    #[inline]
    pub fn is_square(&self) -> bool {
        let diff = self.max - self.min;
        diff.x == diff.y
    }
}

impl Rectangle {
    #[inline]
    fn center(&self) -> IVec2 {
        self.min.mid_point(self.max)
    }

    #[inline]
    fn left(&self) -> i32 {
        self.min.x.min(self.max.x)
    }

    #[inline]
    fn right(&self) -> i32 {
        self.min.x.max(self.max.x)
    }

    #[inline]
    fn top(&self) -> i32 {
        self.min.y.max(self.max.y)
    }

    #[inline]
    fn bottom(&self) -> i32 {
        self.min.y.min(self.max.y)
    }

    /// Check if this rectangle intersects another rectangle.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        // (self.min.cmple(other.max) & self.max.cmpge(other.min)).all()
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Calls a function for each x/y point in the rectangle
    pub fn for_each<F>(&self, f: F)
    where
        F: FnMut(IVec2),
    {
        RectIter::new(self.min, self.max).for_each(f);
    }
}

impl IntoIterator for Rectangle {
    type IntoIter = RectIter;
    type Item = IVec2;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RectIter::new(self.min, self.max)
    }
}
