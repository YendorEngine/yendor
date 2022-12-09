use std::fmt::{Display, Debug};

use crate::prelude::*;

#[derive(Default, Reflect, FromReflect, Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct LocalPosition(UVec2);

impl LocalPosition {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    #[inline(always)]
    pub const fn new(x: u32, y: u32) -> Self { Self(UVec2::new(x, y)) }

    /// Creates a `LocalPostion` with `x` and `y` set to `v`.
    #[inline]
    pub const fn splat(v: u32) -> Self { Self(UVec2::splat(v)) }

    ///////////////////////////////
    /// Getters
    ///////////////////////////////
    #[inline]
    pub const fn x(&self) -> u32 { *self.x }

    #[inline]
    pub const fn y(&self) -> u32 { *self.y }

    #[inline]
    pub const fn gridpoint(&self) -> UVec2 { *self }

    #[inline(always)]
    pub fn grid_index(&self, size: UVec2) -> Option<usize> { *self.as_index(size) }

    ///////////////////////////////
    /// Setters
    ///////////////////////////////
    pub fn set_x(&mut self, value: u32) { *self.x = value; }

    pub fn set_y(&mut self, value: u32) { *self.y = value; }

    pub fn set(&mut self, x: u32, y: u32) {
        self.set_x(x);
        self.set_y(y);
    }
}

impl Display for LocalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalPosition({}, {})", self.x(), self.y())
    }
}

impl Debug for LocalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}