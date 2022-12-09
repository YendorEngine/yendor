use std::fmt::{Debug, Display};

use crate::prelude::*;

/// [`LocalPosition`] is used as an identifier for which point on a `grid` or `chunk` an object belongs to.
#[derive(Default, Reflect, FromReflect, Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct LocalPosition(UVec2);

impl LocalPosition {
    /// Returns [`LocalPosition`] `(0, 0)`.
    pub const ZERO: Self = Self::splat(0);

    /// Creates a new [`LocalPosition`] from `x` and `y`.
    #[inline(always)]
    pub const fn new(x: u32, y: u32) -> Self {
        Self(UVec2::new(x, y))
    }

    /// Creates a new [`LocalPosition`] with all elements set to `value`.
    #[inline]
    pub const fn splat(value: u32) -> Self {
        Self(UVec2::splat(value))
    }

    //#############################
    // Getters
    //#############################

    /// Returns the current `X` value.
    #[inline]
    pub const fn x(&self) -> u32 {
        self.0.x
    }

    /// Returns the current `Y` value.
    #[inline]
    pub const fn y(&self) -> u32 {
        self.0.y
    }

    /// Returns the current `X` and `Y` value as an [`IVec2`].
    #[inline]
    pub const fn gridpoint(&self) -> UVec2 {
        self.0
    }

    #[inline(always)]
    pub fn grid_index(&self, size: UVec2) -> Option<usize> {
        (**self).as_index(size)
    }

    //#############################
    // Setters
    //#############################

    /// Sets the current `X` value.
    pub fn set_x(&mut self, value: u32) {
        self.x = value;
    }

    /// Sets the current `Y` value.
    pub fn set_y(&mut self, value: u32) {
        self.y = value;
    }

    /// Sets the current `X` and `Y` value.
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
