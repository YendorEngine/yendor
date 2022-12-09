use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::prelude::*;

/// Suggested to import this object like:
/// `use yendor::types::Position as YendorPosition`
/// and then wrap it with your own type:
/// `pub type Position = YendorPosition<GRID_WIDTH, GRID_HEIGHT>`
#[derive(Default, Reflect, FromReflect, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Position<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    world_position: WorldPosition,
    local_position: LocalPosition,
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Position<GRID_WIDTH, GRID_HEIGHT> {
    #[inline(always)]
    pub const fn new(world_position: WorldPosition, local_position: LocalPosition) -> Self {
        Self {
            world_position,
            local_position,
        }
    }

    pub const fn new_grid_min(world_position: WorldPosition) -> Self {
        Self {
            world_position,
            local_position: LocalPosition::new(0, 0),
        }
    }

    pub const fn new_grid_max(world_position: WorldPosition) -> Self {
        Self {
            world_position,
            local_position: LocalPosition::new(GRID_WIDTH, GRID_HEIGHT),
        }
    }

    #[inline(always)]
    pub fn as_index(&self) -> usize {
        self.gridpoint().as_index_unchecked(GRID_WIDTH)
    }

    #[inline(always)]
    pub fn as_index_for(&self, size: impl Dimensions) -> Option<usize> {
        self.gridpoint().as_index(size)
    }

    pub fn distance(&self, other: Self) -> u32 {
        let dist_x = self.distance_x(other);
        let dist_y = self.distance_y(other);

        dist_x.max(dist_y)
    }

    pub const fn distance_x(&self, other: Self) -> u32 {
        ((other.world_x() * GRID_WIDTH as i32 + other.x() as i32)
            - (self.world_x() * GRID_WIDTH as i32 + self.x() as i32))
            .unsigned_abs()
    }

    pub const fn distance_y(&self, other: Self) -> u32 {
        ((other.world_y() * GRID_HEIGHT as i32 + other.y() as i32)
            - (self.world_y() * GRID_HEIGHT as i32 + self.y() as i32))
            .unsigned_abs()
    }

    pub const fn octant_to(&self, other: Self) -> Octant<GRID_WIDTH, GRID_HEIGHT> {
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

    pub fn angle_to(&self, other: Self) -> f64 {
        let x = (other.absolute_x() - self.absolute_x()) as f64;
        let y = (other.absolute_y() - self.absolute_y()) as f64;
        y.atan2(x).to_degrees()
    }

    pub fn lerp(&self, other: Self, percent: f32) -> Self {
        let (abs_self_x, abs_self_y, abs_self_z) = self.absolute_position();
        let (abs_other_x, abs_other_y, abs_other_z) = other.absolute_position();

        let lerp_x =
            ((abs_other_x - abs_self_x) as f64).mul_add(percent as f64, abs_self_x as f64) as i64;
        let lerp_y =
            ((abs_other_y - abs_self_y) as f64).mul_add(percent as f64, abs_self_y as f64) as i64;
        let lerp_z =
            ((abs_other_z - abs_self_z) as f64).mul_add(percent as f64, abs_self_z as f64) as i32;

        Self::from_absolute_position((lerp_x, lerp_y, lerp_z))
    }

    ///////////////////////////////
    /// LocalPosition
    ///////////////////////////////
    #[inline]
    pub const fn get_local_position(&self) -> LocalPosition {
        self.local_position
    }

    #[inline]
    pub const fn x(&self) -> u32 {
        self.local_position.x()
    }

    #[inline]
    pub const fn y(&self) -> u32 {
        self.local_position.y()
    }

    #[inline]
    pub const fn gridpoint(&self) -> UVec2 {
        self.local_position.gridpoint()
    }

    pub fn set_x(&mut self, value: u32) {
        self.local_position.set_x(value);
    }

    pub fn add_x(&mut self, value: i32) {
        Self::add_assign(self, IVec2::new(value, 0));
    }

    pub fn sub_x(&mut self, value: i32) {
        Self::sub_assign(self, IVec2::new(value, 0));
    }

    pub fn set_y(&mut self, value: u32) {
        self.local_position.set_y(value);
    }

    pub fn add_y(&mut self, value: i32) {
        Self::add_assign(self, IVec2::new(0, value));
    }

    pub fn sub_y(&mut self, value: i32) {
        Self::sub_assign(self, IVec2::new(0, value));
    }

    pub fn set_xy(&mut self, value: UVec2) {
        self.local_position.set(value.x, value.y);
    }

    pub fn add_xy(&mut self, x: i32, y: i32) {
        Self::add_assign(self, IVec2::new(x, y));
    }

    pub fn sub_xy(&mut self, x: i32, y: i32) {
        Self::sub_assign(self, IVec2::new(x, y));
    }

    ///////////////////////////////
    /// WorldPosition
    ///////////////////////////////
    #[inline]
    pub const fn get_world_position(&self) -> WorldPosition {
        self.world_position
    }

    #[inline]
    pub const fn world_x(&self) -> i32 {
        self.world_position.x()
    }

    #[inline]
    pub const fn world_y(&self) -> i32 {
        self.world_position.y()
    }

    #[inline]
    pub const fn world_z(&self) -> i32 {
        self.world_position.z()
    }

    #[inline]
    pub const fn world_xyz(&self) -> IVec3 {
        self.world_position.xyz()
    }

    pub fn set_world_x(&mut self, value: i32) {
        self.world_position.set_x(value);
    }

    pub fn set_world_y(&mut self, value: i32) {
        self.world_position.set_y(value);
    }

    pub fn set_world_z(&mut self, value: i32) {
        self.world_position.set_z(value);
    }

    pub fn set_world_xy(&mut self, value: IVec2) {
        self.world_position.set_xy(value.x, value.y);
    }

    pub fn set_world_xyz(&mut self, value: IVec3) {
        self.world_position.set(value.x, value.y, value.z);
    }

    ///////////////////////////////
    /// Expert
    ///////////////////////////////
    pub const fn absolute_position(&self) -> (i64, i64, i32) {
        (self.absolute_x(), self.absolute_y(), self.world_z())
    }

    pub const fn absolute_x(&self) -> i64 {
        self.world_x() as i64 * GRID_WIDTH as i64 + self.x() as i64
    }

    pub const fn absolute_y(&self) -> i64 {
        self.world_y() as i64 * GRID_HEIGHT as i64 + self.y() as i64
    }

    pub const fn from_absolute_position(absolute_position: (i64, i64, i32)) -> Self {
        let (world_x, local_x) = if absolute_position.0 < 0 {
            let abs_x = absolute_position.0.abs();
            let mut world = abs_x / GRID_WIDTH as i64;
            let mut local = GRID_WIDTH as i64 - (abs_x - (world * GRID_WIDTH as i64));
            if local == GRID_WIDTH as i64 {
                world -= 1;
                local = 0;
            }
            (-(world as i32) - 1, local as u32)
        } else {
            (
                (absolute_position.0 / GRID_WIDTH as i64) as i32,
                (absolute_position.0 % GRID_WIDTH as i64) as u32,
            )
        };

        let (world_y, local_y) = if absolute_position.1 < 0 {
            let abs_y = absolute_position.1.abs();
            let mut world = abs_y / GRID_HEIGHT as i64;
            let mut local = GRID_HEIGHT as i64 - (abs_y - (world * GRID_HEIGHT as i64));
            if local == GRID_HEIGHT as i64 {
                world -= 1;
                local = 0;
            }
            (-(world as i32) - 1, local as u32)
        } else {
            (
                (absolute_position.1 / GRID_HEIGHT as i64) as i32,
                (absolute_position.1 % GRID_HEIGHT as i64) as u32,
            )
        };

        Self::new(
            WorldPosition::new(world_x, world_y, absolute_position.2),
            LocalPosition::new(local_x, local_y),
        )
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> PartialEq
    for Position<GRID_WIDTH, GRID_HEIGHT>
{
    fn eq(&self, other: &Self) -> bool {
        self.world_position == other.world_position
            && self.get_local_position() == other.get_local_position()
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Eq for Position<GRID_WIDTH, GRID_HEIGHT> {}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Hash for Position<GRID_WIDTH, GRID_HEIGHT> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        GRID_WIDTH.hash(state);
        GRID_HEIGHT.hash(state);
        self.world_position.hash(state);
        self.get_local_position().hash(state);
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Display for Position<GRID_WIDTH, GRID_HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position{{\n\t{}\n\t{}\n}}",
            self.world_position, self.local_position,
        )
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Debug for Position<GRID_WIDTH, GRID_HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}::{:?})", self.world_position, self.local_position)
    }
}

// Add offset to LocalPosition
impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Add<IVec2>
    for Position<GRID_WIDTH, GRID_HEIGHT>
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: IVec2) -> Self::Output {
        let mut world_x = self.world_x();
        let mut world_y = self.world_y();

        let mut local_x = self.x() as i32 + rhs.x;
        let mut local_y = self.y() as i32 + rhs.y;

        if local_x < 0 {
            world_x -= 1;
            local_x += GRID_WIDTH as i32;
        } else if local_x >= GRID_WIDTH as i32 {
            world_x += 1;
            local_x -= GRID_WIDTH as i32;
        }

        if local_y < 0 {
            world_y -= 1;
            local_y += GRID_HEIGHT as i32;
        } else if local_y >= GRID_HEIGHT as i32 {
            world_y += 1;
            local_y -= GRID_HEIGHT as i32;
        }

        Self::new(
            WorldPosition::new(world_x, world_y, self.world_z()),
            LocalPosition::new(local_x as u32, local_y as u32),
        )
    }
}

// Add offset to LocalPosition
impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> AddAssign<IVec2>
    for Position<GRID_WIDTH, GRID_HEIGHT>
{
    #[inline]
    fn add_assign(&mut self, rhs: IVec2) {
        let new_x = self.x() as i32 + rhs.x;
        let new_y = self.y() as i32 + rhs.y;

        if new_x < 0 {
            self.set_world_x(self.world_x() - 1);
            self.set_x((new_x + GRID_WIDTH as i32) as u32);
        } else if new_x >= GRID_WIDTH as i32 {
            self.set_world_x(self.world_x() + 1);
            self.set_x((new_x - GRID_WIDTH as i32) as u32);
        } else {
            self.set_x(new_x as u32);
        }

        if new_y < 0 {
            self.set_world_y(self.world_y() - 1);
            self.set_y((new_y + GRID_HEIGHT as i32) as u32);
        } else if new_y >= GRID_HEIGHT as i32 {
            self.set_world_y(self.world_y() + 1);
            self.set_y((new_y - GRID_HEIGHT as i32) as u32);
        } else {
            self.set_y(new_y as u32);
        }
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Add<Direction>
    for Position<GRID_WIDTH, GRID_HEIGHT>
{
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let coord = rhs.coord();
        self + coord
    }
}

// Sub offset to LocalPosition
impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Sub<IVec2>
    for Position<GRID_WIDTH, GRID_HEIGHT>
{
    type Output = Self;

    fn sub(self, rhs: IVec2) -> Self::Output {
        let mut world_x = self.world_x();
        let mut world_y = self.world_y();

        let mut local_x = self.x() as i32 - rhs.x;
        let mut local_y = self.y() as i32 - rhs.y;

        if local_x < 0 {
            world_x -= 1;
            local_x += GRID_WIDTH as i32;
        } else if local_x >= GRID_WIDTH as i32 {
            world_x += 1;
            local_x -= GRID_WIDTH as i32;
        }

        if local_y < 0 {
            world_y -= 1;
            local_y += GRID_HEIGHT as i32;
        } else if local_y >= GRID_HEIGHT as i32 {
            world_y += 1;
            local_y -= GRID_HEIGHT as i32;
        }

        Self::new(
            WorldPosition::new(world_x, world_y, self.world_z()),
            LocalPosition::new(local_x as u32, local_y as u32),
        )
    }
}

// Sub offset to LocalPosition
impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> SubAssign<IVec2>
    for Position<GRID_WIDTH, GRID_HEIGHT>
{
    fn sub_assign(&mut self, rhs: IVec2) {
        let new_x = self.x() as i32 - rhs.x;
        let new_y = self.y() as i32 - rhs.y;

        if new_x < 0 {
            self.set_world_x(self.world_x() - 1);
            self.set_x((new_x + GRID_WIDTH as i32) as u32);
        } else if new_x >= GRID_WIDTH as i32 {
            self.set_world_x(self.world_x() + 1);
            self.set_x((new_x - GRID_WIDTH as i32) as u32);
        } else {
            self.set_x(new_x as u32);
        }

        if new_y < 0 {
            self.set_world_y(self.world_y() - 1);
            self.set_y((new_y + GRID_HEIGHT as i32) as u32);
        } else if new_y >= GRID_HEIGHT as i32 {
            self.set_world_y(self.world_y() + 1);
            self.set_y((new_y - GRID_HEIGHT as i32) as u32);
        } else {
            self.set_y(new_y as u32);
        }
    }
}
