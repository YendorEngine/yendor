use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::prelude::*;

/// [`Position`] is used to locate objects across multiple grid-like maps. The
/// [`WorldPosition`] part is automatically updated using any of the `Add` or `Sub` functions.
/// This allows `distance` and other things to also be performed across multiple `grid`s.
///
/// Suggested to import this object like:
///
/// `use yendor::types::Position as YendorPosition`
///
/// and then wrap it with your own type:
///
/// `pub type Position = YendorPosition<DIM>`
#[derive(Default, Clone, Copy)]
#[cfg_attr(feature = "reflect", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Position<const DIM: UVec2> {
    world_position: WorldPosition,
    local_position: LocalPosition,
}

impl<const DIM: UVec2> Position<DIM> {
    /// Returns [`Position`] `(0, 0)`.
    pub const ZERO: Self = Self::splat(0);

    /// Creates a new [`Position`] from a [`WorldPosition`] and a [`LocalPosition`].
    #[inline(always)]
    pub const fn new(world_position: WorldPosition, local_position: LocalPosition) -> Self {
        Self {
            world_position,
            local_position,
        }
    }

    /// Creates a new [`Position`] with all elements set to `value`.
    #[inline]
    pub const fn splat(value: u32) -> Self {
        Self {
            local_position: LocalPosition::splat(value),
            world_position: WorldPosition::splat(value as i32),
        }
    }

    /// Creates a new [`Position`] from a [`WorldPosition`] and the [`LocalPosition`] set to
    /// `(0, 0)`
    pub const fn new_grid_min(world_position: WorldPosition) -> Self {
        Self {
            world_position,
            local_position: LocalPosition::new(0, 0),
        }
    }

    /// Creates a new[`Position`] from a [`WorldPosition`] and the [`LocalPosition`] set to
    /// `(DIM.x - 1, DIM.y - 1)`
    pub const fn new_grid_max(world_position: WorldPosition) -> Self {
        Self {
            world_position,
            local_position: LocalPosition::new(DIM.x - 1, DIM.y - 1),
        }
    }

    /// Returns an index composed from [`LocalPosition`] for a `grid` with size `(DIM.x,
    /// DIM.y)`
    #[inline(always)]
    pub fn as_index(&self) -> usize { self.gridpoint().as_index_unchecked(DIM.x) }

    /// Returns an Option<index> composed from [`LocalPosition`] for a `grid` with a custom
    /// size.
    ///
    /// NOTE: You probably want [`as_index()`]
    #[inline(always)]
    pub fn as_index_for(&self, size: impl Dimensions) -> Option<usize> { self.gridpoint().as_index(size) }

    /// Computes the distance between two [`Position`]s.
    ///
    /// NOTE: If you can guarantee both [`Position`]s have the same [`WorldPosition`], it may
    /// be cheaper to compute the distance between the [`LocalPosition`]s. ```
    /// let distance = self.get_local_position().distance(other.get_local_position());
    /// ```
    pub fn distance(&self, other: Self) -> u32 {
        let dist_x = self.distance_x(other);
        let dist_y = self.distance_y(other);

        dist_x.max(dist_y)
    }

    /// Computes the distance between two [`Position`]'s `X` value.
    ///
    /// NOTE: If you can guarantee both [`Position`]s have the same [`WorldPosition`], it may
    /// be cheaper to compute the distance between the [`LocalPosition`]'s `X` value. ```
    /// let distance = (other.get_local_position().x() - self.get_local_position().x()).abs();
    /// ```
    pub const fn distance_x(&self, other: Self) -> u32 {
        ((other.world_x() as i64 * DIM.x as i64 + other.x() as i64) -
            (self.world_x() as i64 * DIM.y as i64 + self.x() as i64))
            .unsigned_abs() as u32
    }

    /// Computes the distance between two [`Position`]'s `Y` value.
    ///
    /// NOTE: If you can guarantee both [`Position`]s have the same [`WorldPosition`], it may
    /// be cheaper to compute the distance between the [`LocalPosition`]'s `Y` value. ```
    /// let distance = (other.get_local_position().y() - self.get_local_position().y()).abs();
    /// ```
    pub const fn distance_y(&self, other: Self) -> u32 {
        ((other.world_y() as i64 * DIM.x as i64 + other.y() as i64) -
            (self.world_y() as i64 * DIM.y as i64 + self.y() as i64))
            .unsigned_abs() as u32
    }

    /// Creates the [`octant`] to which the `other` [`Position`] belongs relative to this
    /// [`Position`]. This is useful for transforming static offsets in a dynamic direction.
    pub const fn octant_to(&self, other: Self) -> Octant<DIM> {
        // adapted from <http://codereview.stackexchange.com/a/95551>
        let start = self.absolute_position();
        let end = other.absolute_position();

        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;
        let mut octant = 0;
        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }
        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2;
        }
        if dx < dy {
            octant += 1;
        }

        Octant(octant)
    }

    /// Returns the angle in degrees between two [`Position`]s where `East` is 0deg.
    pub fn angle_to(&self, other: Self) -> f64 {
        let x = (other.absolute_x() - self.absolute_x()) as f64;
        let y = (other.absolute_y() - self.absolute_y()) as f64;
        y.atan2(x).to_degrees()
    }

    /// Linearly interpolates between two [`Position`]s by `percent`
    ///
    /// if `percent = 0.0` return is `self`
    ///
    /// if `percent = 1.0` return is `other`
    ///
    /// NOTE: Unclamped
    pub fn lerp(&self, other: Self, percent: f32) -> Self {
        let (abs_self_x, abs_self_y, abs_self_z) = self.absolute_position();
        let (abs_other_x, abs_other_y, abs_other_z) = other.absolute_position();

        let lerp_x = ((abs_other_x - abs_self_x) as f64).mul_add(percent as f64, abs_self_x as f64) as i64;
        let lerp_y = ((abs_other_y - abs_self_y) as f64).mul_add(percent as f64, abs_self_y as f64) as i64;
        let lerp_z = ((abs_other_z - abs_self_z) as f64).mul_add(percent as f64, abs_self_z as f64) as i32;

        Self::from_absolute_position((lerp_x, lerp_y, lerp_z))
    }

    //#############################
    // LocalPosition
    //#############################

    /// Returns the current [`LocalPosition`]
    #[inline]
    pub const fn get_local_position(&self) -> LocalPosition { self.local_position }

    /// Returns the current [`LocalPostion`]'s `X`
    #[inline]
    pub const fn x(&self) -> u32 { self.local_position.x() }

    /// Returns the current [`LocalPostion`]'s `Y`
    #[inline]
    pub const fn y(&self) -> u32 { self.local_position.y() }

    /// Returns the current [`LocalPostion`] as a [`UVec2`]
    #[inline]
    pub const fn gridpoint(&self) -> UVec2 { self.local_position.gridpoint() }

    /// Sets the current [`LocalPostion`]'s `X`
    /// Does NOT adjust [`WorldPosition`]
    pub fn set_x(&mut self, value: u32) { self.local_position.set_x(value); }

    /// Adds a value to the current [`LocalPostion`]'s `X`
    /// Adjusts [`WorldPosition`] as necessary
    pub fn add_x(&mut self, value: i32) { Self::add_assign(self, IVec2::new(value, 0)); }

    /// Subtracts a value from the current [`LocalPostion`]'s `X`
    /// Adjusts [`WorldPosition`] as necessary
    pub fn sub_x(&mut self, value: i32) { Self::sub_assign(self, IVec2::new(value, 0)); }

    /// Sets the current [`LocalPostion`]'s `Y`
    /// Does NOT adjust [`WorldPosition`]
    pub fn set_y(&mut self, value: u32) { self.local_position.set_y(value); }

    /// Adds a value to the current [`LocalPostion`]'s `Y`
    /// Adjusts [`WorldPosition`] as necessary
    pub fn add_y(&mut self, value: i32) { Self::add_assign(self, IVec2::new(0, value)); }

    /// Subtracts a value from the current [`LocalPostion`]'s `X`
    /// Adjusts [`WorldPosition`] as necessary
    pub fn sub_y(&mut self, value: i32) { Self::sub_assign(self, IVec2::new(0, value)); }

    /// Sets the current [`LocalPosition`]'s `X` and `Y`
    /// Does NOT adjust [`WorldPosition`]
    pub fn set_xy(&mut self, value: UVec2) { self.local_position.set(value.x, value.y); }

    /// Adds a value to the current [`LocalPosition`]'s `X` and `Y`
    /// Does NOT adjust [`WorldPosition`]
    pub fn add_xy(&mut self, x: i32, y: i32) { Self::add_assign(self, IVec2::new(x, y)); }

    /// Subtracts a value from the current [`LocalPosition`]'s `X` and `Y`
    /// Does NOT adjust [`WorldPosition`]
    pub fn sub_xy(&mut self, x: i32, y: i32) { Self::sub_assign(self, IVec2::new(x, y)); }

    //#############################
    // WorldPosition
    //#############################

    /// Returns the current [`WorldPosition`]
    #[inline]
    pub const fn get_world_position(&self) -> WorldPosition { self.world_position }

    /// Returns the current [`WorldPosition`]'s `X`
    #[inline]
    pub const fn world_x(&self) -> i32 { self.world_position.x() }

    /// Returns the current [`WorldPosition`]'s `Y`
    #[inline]
    pub const fn world_y(&self) -> i32 { self.world_position.y() }

    /// Returns the current [`WorldPosition`]'s `Z`
    #[inline]
    pub const fn world_z(&self) -> i32 { self.world_position.z() }

    /// Returns the current [`WorldPosition`] as an [`IVec3`]
    #[inline]
    pub const fn world_xyz(&self) -> IVec3 { self.world_position.xyz() }

    /// Sets the current [`WorldPosition`]'s `X`
    pub fn set_world_x(&mut self, value: i32) { self.world_position.set_x(value); }

    /// Sets the current [`WorldPosition`]'s `Y`
    pub fn set_world_y(&mut self, value: i32) { self.world_position.set_y(value); }

    /// Sets the current [`WorldPosition`]'s `Z`
    pub fn set_world_z(&mut self, value: i32) { self.world_position.set_z(value); }

    /// Sets the current [`WorldPosition`]'s `X` and `Y`
    pub fn set_world_xy(&mut self, value: IVec2) { self.world_position.set_xy(value.x, value.y); }

    /// Sets the current [`WorldPosition`]'s `X`, `Y`, and `Z`
    pub fn set_world_xyz(&mut self, value: IVec3) { self.world_position.set(value.x, value.y, value.z); }

    //#############################
    // Absolute
    //#############################

    /// Returns the current [`WorldPosition`] and [`LocalPosition`] calculated out and returned
    /// as `(X, Y, Z)`
    pub const fn absolute_position(&self) -> (i64, i64, i32) {
        (self.absolute_x(), self.absolute_y(), self.world_z())
    }

    /// Returns the current [`WorldPosition`]'s `X` and [`LocalPosition`]'s `X` calculated out
    pub const fn absolute_x(&self) -> i64 { self.world_x() as i64 * DIM.x as i64 + self.x() as i64 }

    /// Returns the current [`WorldPosition`]'s `X` and [`LocalPosition`]'s `Y` calculated out
    pub const fn absolute_y(&self) -> i64 { self.world_y() as i64 * DIM.y as i64 + self.y() as i64 }

    /// Returns a [`Position`] created from an `absolute position`
    pub const fn from_absolute_position(absolute_position: (i64, i64, i32)) -> Self {
        let (world_x, local_x) = if absolute_position.0 < 0 {
            let abs_x = absolute_position.0.abs();
            let mut world = abs_x / DIM.x as i64;
            let mut local = DIM.x as i64 - (abs_x - (world * DIM.x as i64));
            if local == DIM.x as i64 {
                world -= 1;
                local = 0;
            }
            (-(world as i32) - 1, local as u32)
        } else {
            (
                (absolute_position.0 / DIM.x as i64) as i32,
                (absolute_position.0 % DIM.x as i64) as u32,
            )
        };

        let (world_y, local_y) = if absolute_position.1 < 0 {
            let abs_y = absolute_position.1.abs();
            let mut world = abs_y / DIM.y as i64;
            let mut local = DIM.y as i64 - (abs_y - (world * DIM.y as i64));
            if local == DIM.y as i64 {
                world -= 1;
                local = 0;
            }
            (-(world as i32) - 1, local as u32)
        } else {
            (
                (absolute_position.1 / DIM.y as i64) as i32,
                (absolute_position.1 % DIM.y as i64) as u32,
            )
        };

        Self::new(
            WorldPosition::new(world_x, world_y, absolute_position.2),
            LocalPosition::new(local_x, local_y),
        )
    }
}

impl<const DIM: UVec2> PartialEq for Position<DIM> {
    fn eq(&self, other: &Self) -> bool {
        self.world_position == other.world_position && self.get_local_position() == other.get_local_position()
    }
}

impl<const DIM: UVec2> Eq for Position<DIM> {}

impl<const DIM: UVec2> Hash for Position<DIM> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        DIM.x.hash(state);
        DIM.y.hash(state);
        self.world_position.hash(state);
        self.get_local_position().hash(state);
    }
}

impl<const DIM: UVec2> Display for Position<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position{{\n\t{}\n\t{}\n}}",
            self.world_position, self.local_position,
        )
    }
}

impl<const DIM: UVec2> Debug for Position<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}::{:?})", self.world_position, self.local_position)
    }
}

// TODO: Fix IVec2 > DIM.x, DIM.y
// Add offset to LocalPosition
impl<const DIM: UVec2> Add<IVec2> for Position<DIM> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: IVec2) -> Self::Output {
        let mut world_x = self.world_x();
        let mut world_y = self.world_y();

        let mut local_x = self.x() as i32 + rhs.x;
        let mut local_y = self.y() as i32 + rhs.y;

        if local_x < 0 {
            world_x -= 1;
            local_x += DIM.x as i32;
        } else if local_x >= DIM.x as i32 {
            world_x += 1;
            local_x -= DIM.x as i32;
        }

        if local_y < 0 {
            world_y -= 1;
            local_y += DIM.y as i32;
        } else if local_y >= DIM.y as i32 {
            world_y += 1;
            local_y -= DIM.y as i32;
        }

        Self::new(
            WorldPosition::new(world_x, world_y, self.world_z()),
            LocalPosition::new(local_x as u32, local_y as u32),
        )
    }
}

// TODO: Fix IVec2 > DIM.x, DIM.y
// Add offset to LocalPosition
impl<const DIM: UVec2> AddAssign<IVec2> for Position<DIM> {
    #[inline]
    fn add_assign(&mut self, rhs: IVec2) {
        let new_x = self.x() as i32 + rhs.x;
        let new_y = self.y() as i32 + rhs.y;

        if new_x < 0 {
            self.set_world_x(self.world_x() - 1);
            self.set_x((new_x + DIM.x as i32) as u32);
        } else if new_x >= DIM.x as i32 {
            self.set_world_x(self.world_x() + 1);
            self.set_x((new_x - DIM.x as i32) as u32);
        } else {
            self.set_x(new_x as u32);
        }

        if new_y < 0 {
            self.set_world_y(self.world_y() - 1);
            self.set_y((new_y + DIM.y as i32) as u32);
        } else if new_y >= DIM.y as i32 {
            self.set_world_y(self.world_y() + 1);
            self.set_y((new_y - DIM.y as i32) as u32);
        } else {
            self.set_y(new_y as u32);
        }
    }
}

impl<const DIM: UVec2> Add<Direction> for Position<DIM> {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let coord = rhs.coord();
        self + coord
    }
}

// TODO: Fix IVec2 > DIM.x, DIM.y
// Sub offset to LocalPosition
impl<const DIM: UVec2> Sub<IVec2> for Position<DIM> {
    type Output = Self;

    fn sub(self, rhs: IVec2) -> Self::Output {
        let mut world_x = self.world_x();
        let mut world_y = self.world_y();

        let mut local_x = self.x() as i32 - rhs.x;
        let mut local_y = self.y() as i32 - rhs.y;

        if local_x < 0 {
            world_x -= 1;
            local_x += DIM.x as i32;
        } else if local_x >= DIM.x as i32 {
            world_x += 1;
            local_x -= DIM.x as i32;
        }

        if local_y < 0 {
            world_y -= 1;
            local_y += DIM.y as i32;
        } else if local_y >= DIM.y as i32 {
            world_y += 1;
            local_y -= DIM.y as i32;
        }

        Self::new(
            WorldPosition::new(world_x, world_y, self.world_z()),
            LocalPosition::new(local_x as u32, local_y as u32),
        )
    }
}

// TODO: Fix IVec2 > DIM.x, DIM.y
// Sub offset to LocalPosition
impl<const DIM: UVec2> SubAssign<IVec2> for Position<DIM> {
    fn sub_assign(&mut self, rhs: IVec2) {
        let new_x = self.x() as i32 - rhs.x;
        let new_y = self.y() as i32 - rhs.y;

        if new_x < 0 {
            self.set_world_x(self.world_x() - 1);
            self.set_x((new_x + DIM.x as i32) as u32);
        } else if new_x >= DIM.x as i32 {
            self.set_world_x(self.world_x() + 1);
            self.set_x((new_x - DIM.x as i32) as u32);
        } else {
            self.set_x(new_x as u32);
        }

        if new_y < 0 {
            self.set_world_y(self.world_y() - 1);
            self.set_y((new_y + DIM.y as i32) as u32);
        } else if new_y >= DIM.y as i32 {
            self.set_world_y(self.world_y() + 1);
            self.set_y((new_y - DIM.y as i32) as u32);
        } else {
            self.set_y(new_y as u32);
        }
    }
}
