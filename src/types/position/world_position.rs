use std::fmt::{Display, Debug};

use crate::prelude::*;

#[derive(Default, Reflect, FromReflect, Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct WorldPosition(IVec3);

impl WorldPosition {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    #[inline(always)]
    pub const fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

    /// Creates a `WorldPosition` with all elements set to `v`.
    #[inline]
    pub const fn splat(v: i32) -> Self { Self(IVec3::splat(v)) }

    ///////////////////////////////
    /// Getters
    ///////////////////////////////
    #[inline]
    pub const fn x(&self) -> i32 { self.0.x }

    #[inline]
    pub const fn y(&self) -> i32 { self.0.y }

    #[inline]
    pub const fn z(&self) -> i32 { self.0.z }

    #[inline]
    pub const fn xy(&self) -> IVec2 { IVec2::new(self.x(), self.y()) }

    #[inline]
    pub const fn xyz(&self) -> IVec3 { self.0 }

    ///////////////////////////////
    /// Setters
    ///////////////////////////////
    pub fn set_x(&mut self, value: i32) { self.x = value; }

    pub fn set_y(&mut self, value: i32) { self.y = value; }

    pub fn set_z(&mut self, value: i32) { self.z = value; }

    pub fn set_xy(&mut self, x: i32, y: i32) {
        self.set_x(x);
        self.set_y(y);
    }

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