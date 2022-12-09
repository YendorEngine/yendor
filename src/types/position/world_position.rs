use std::fmt::{Debug, Display};

use crate::prelude::*;

/// [`WorldPosition`] is used as an identifier for which `grid` or `chunk` an object belongs to.
#[derive(Default, Reflect, FromReflect, Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct WorldPosition(IVec3);

impl WorldPosition {
    /// Returns [`WorldPosition`] `(0, 0, 0)`.
    pub const ZERO: Self = Self::splat(0);

    /// Creates a new [`WorldPosition`] from `x`, `y`, and `z`.
    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    /// Creates a new [`WorldPosition`] with all elements set to `value`.
    #[inline]
    pub const fn splat(value: i32) -> Self {
        Self(IVec3::splat(value))
    }

    //#############################
    // Getters
    //#############################

    /// Returns the current `X` value.
    #[inline]
    pub const fn x(&self) -> i32 {
        self.0.x
    }

    /// Returns the current `Y` value.
    #[inline]
    pub const fn y(&self) -> i32 {
        self.0.y
    }

    /// Returns the current `Z` value.
    #[inline]
    pub const fn z(&self) -> i32 {
        self.0.z
    }

    /// Returns the current `X` and `Y` value as an [`IVec2`].
    #[inline]
    pub const fn xy(&self) -> IVec2 {
        IVec2::new(self.x(), self.y())
    }

    /// Returns the current `X`, `Y`, and `Z` value as an [`IVec3`].
    #[inline]
    pub const fn xyz(&self) -> IVec3 {
        self.0
    }

    //#############################
    // Setters
    //#############################

    /// Sets the current `X` value.
    pub fn set_x(&mut self, value: i32) {
        self.x = value;
    }

    /// Sets the current `Y` value.
    pub fn set_y(&mut self, value: i32) {
        self.y = value;
    }

    /// Sets the current `Z` value.
    pub fn set_z(&mut self, value: i32) {
        self.z = value;
    }

    /// Sets the current `X` and `Y` value.
    pub fn set_xy(&mut self, x: i32, y: i32) {
        self.set_x(x);
        self.set_y(y);
    }

    /// Sets the current `X`, `Y`, and `Z` value.
    pub fn set(&mut self, x: i32, y: i32, z: i32) {
        self.set_xy(x, y);
        self.set_z(z);
    }
}

impl Display for WorldPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WorldPosition({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl Debug for WorldPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
