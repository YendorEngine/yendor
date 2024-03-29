//! Provides an api for working with directions in grid-space.
//!
//! Usage:
//!
//! `Direction`s can be selected by:
//! ```no_run
//! use yendor_rl::prelude::*;
//!
//! let north = Direction::NORTH;
//! ```
//! where `NORTH` is the direction you want. You may limit the list available by using
//! `YYYDirection::XXX` where `YYY` is one of: `Cardinal / Ordinal / Vertical`
//!
//! `Direction`s may also be composed by `Add`ing or `Sub`tracting the values:
//! ```no_run
//! use yendor_rl::prelude::*;
//!
//! let some_direction = Direction::NORTH + Direction::EAST; // NORTH_EAST
//! assert_eq!(some_direction, Direction::NORTH_EAST);
//! ```
//!
//! Otherwise invalid directions may be created this way as well; allowing for code such as:
//! ```
//! use bevy::prelude::*;
//! use yendor_rl::prelude::*;
//!
//! fn keyboard_input(keys: Res<Input<KeyCode>>) {
//!     let mut input_direction = Direction::NONE;
//!
//!     if keys.pressed(KeyCode::W) {
//!         input_direction += Direction::NORTH
//!     }
//!
//!     if keys.pressed(KeyCode::S) {
//!        input_direction += Direction::SOUTH
//!     }
//!
//!     if keys.pressed(KeyCode::A) {
//!         input_direction += Direction::WEST
//!     }
//!
//!     if keys.pressed(KeyCode::D) {
//!         input_direction += Direction::EAST
//!     }
//! }
//!
//! ```

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::prelude::*;

mod cardinal_direction;
pub use cardinal_direction::*;
mod direction_flags;
pub(crate) use direction_flags::*;
mod direction_iter;
pub(crate) use direction_iter::*;
mod direction_iterator;
pub use direction_iterator::*;
mod direction_table;
pub(crate) use direction_table::*;
mod direction_type;
pub(crate) use direction_type::*;
mod ordinal_direction;
pub use ordinal_direction::*;
mod vertical_direction;
pub use vertical_direction::*;

/// `Direction` represents a direction in space.
///
/// NOTE: These functions gives preference to `North`, `East`, and `Up` on the source when
/// using `Invalid` [`Direction`]s
#[derive(Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Direction(DirectionType);

impl Direction {
    /// Returns the `Down` [`Direction`]
    pub const DOWN: Self = Self(DirectionFlags::DOWN);

    /// Returns the `DownEast` [`Direction`]
    pub const DOWN_EAST: Self = Self(DirectionFlags::DOWN | DirectionFlags::EAST);

    /// Returns the `DownNorth` [`Direction`]
    pub const DOWN_NORTH: Self = Self(DirectionFlags::DOWN | DirectionFlags::NORTH);

    /// Returns the `DownNorthEast` [`Direction`]
    pub const DOWN_NORTH_EAST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::NORTH | DirectionFlags::EAST);

    /// Returns the `DownNorthWest` [`Direction`]
    pub const DOWN_NORTH_WEST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::NORTH | DirectionFlags::WEST);

    /// Returns the `DownSouth` [`Direction`]
    pub const DOWN_SOUTH: Self = Self(DirectionFlags::DOWN | DirectionFlags::SOUTH);

    /// Returns the `DownSouthEast` [`Direction`]
    pub const DOWN_SOUTH_EAST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::SOUTH | DirectionFlags::EAST);

    /// Returns the `DownSouthWest` [`Direction`]
    pub const DOWN_SOUTH_WEST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::SOUTH | DirectionFlags::WEST);

    /// Returns the `DownWest` [`Direction`]
    pub const DOWN_WEST: Self = Self(DirectionFlags::DOWN | DirectionFlags::WEST);

    /// Returns the `East` [`Direction`]
    pub const EAST: Self = Self(DirectionFlags::EAST);

    /// Returns the `Invalid` [`Direction`]
    pub const NONE: Self = Self(0);

    /// Returns the `North` [`Direction`]
    pub const NORTH: Self = Self(DirectionFlags::NORTH);

    /// Returns the `NorthEast` [`Direction`]
    pub const NORTH_EAST: Self = Self(DirectionFlags::NORTH | DirectionFlags::EAST);

    /// Returns the `NorthWest` [`Direction`]
    pub const NORTH_WEST: Self = Self(DirectionFlags::NORTH | DirectionFlags::WEST);

    /// Returns the `South` [`Direction`]
    pub const SOUTH: Self = Self(DirectionFlags::SOUTH);

    /// Returns the `SouthEast` [`Direction`]
    pub const SOUTH_EAST: Self = Self(DirectionFlags::SOUTH | DirectionFlags::EAST);

    /// Returns the `SouthWest` [`Direction`]
    pub const SOUTH_WEST: Self = Self(DirectionFlags::SOUTH | DirectionFlags::WEST);

    /// Returns the `Up` [`Direction`]
    pub const UP: Self = Self(DirectionFlags::UP);

    /// Returns the `UpEast` [`Direction`]
    pub const UP_EAST: Self = Self(DirectionFlags::UP | DirectionFlags::EAST);

    /// Returns the `UpNorth` [`Direction`]
    pub const UP_NORTH: Self = Self(DirectionFlags::UP | DirectionFlags::NORTH);

    /// Returns the `UpNorthEast` [`Direction`]
    pub const UP_NORTH_EAST: Self =
        Self(DirectionFlags::UP | DirectionFlags::NORTH | DirectionFlags::EAST);

    /// Returns the `UpNorthWest` [`Direction`]
    pub const UP_NORTH_WEST: Self =
        Self(DirectionFlags::UP | DirectionFlags::NORTH | DirectionFlags::WEST);

    /// Returns the `UpSouth` [`Direction`]
    pub const UP_SOUTH: Self = Self(DirectionFlags::UP | DirectionFlags::SOUTH);

    /// Returns the `UpSouthEast` [`Direction`]
    pub const UP_SOUTH_EAST: Self =
        Self(DirectionFlags::UP | DirectionFlags::SOUTH | DirectionFlags::EAST);

    /// Returns the `UpSouthWest` [`Direction`]
    pub const UP_SOUTH_WEST: Self =
        Self(DirectionFlags::UP | DirectionFlags::SOUTH | DirectionFlags::WEST);

    /// Returns the `UpWest` [`Direction`]
    pub const UP_WEST: Self = Self(DirectionFlags::UP | DirectionFlags::WEST);

    /// Returns the `West` [`Direction`]
    pub const WEST: Self = Self(DirectionFlags::WEST);
}

impl Direction {
    /// Retrieves the 2d coordinate value from the [`Direction`] where:
    ///
    /// `East` = `1` on the `X` axis
    ///
    /// `West` = `-1` on the `X` axis
    ///
    /// `North` = `1` on the `Y` axis
    ///
    /// `South` = `-1` on the `Y` axis
    pub const fn coord(self) -> IVec2 {
        let x = if self.has_east() {
            1
        } else if self.has_west() {
            -1
        } else {
            0
        };

        let y = if self.has_north() {
            1
        } else if self.has_south() {
            -1
        } else {
            0
        };

        IVec2::new(x, y)
    }

    /// Retrieves the 3d coordinate value from the [`Direction`] where:
    ///
    /// `East` = `1` on the `X` axis
    ///
    /// `West` = `-1` on the `X` axis
    ///
    /// `North` = `1` on the `Y` axis
    ///
    /// `South` = `-1` on the `Y` axis
    ///
    /// `Up` = `1` on the `Z` axis
    ///
    /// `Down` = `-1` on the `Z` axis
    pub const fn coord3d(self) -> IVec3 {
        let z = if self.has_up() {
            1
        } else if self.has_down() {
            -1
        } else {
            0
        };

        self.coord().extend(z)
    }

    /// Retrieves the [`Direction`] from a 2d coordinate value where:
    ///
    /// `East` = `1` on the `X` axis
    ///
    /// `West` = `-1` on the `X` axis
    ///
    /// `North` = `1` on the `Y` axis
    ///
    /// `South` = `-1` on the `Y` axis
    pub fn from_coord(coord: IVec2) -> Self {
        Self::from_ivec3(coord.extend(0))
    }

    // TODO: Point3d
    /// Retrieves the [`Direction`] from a 3d coordinate value where:
    ///
    /// `East` = `1` on the `X` axis
    ///
    /// `West` = `-1` on the `X` axis
    ///
    /// `North` = `1` on the `Y` axis
    ///
    /// `South` = `-1` on the `Y` axis
    ///
    /// `Up` = `1` on the `Z` axis
    ///
    /// `Down` = `-1` on the `Z` axis
    pub fn from_ivec3(coord: IVec3) -> Self {
        let mut direction = Self::NONE;

        if coord.x > 0 {
            direction += Self::EAST;
        } else if coord.y < 0 {
            direction += Self::WEST;
        }

        // Check Y
        match coord.y.cmp(&0) {
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Less => direction += Self::SOUTH,
            std::cmp::Ordering::Greater => direction += Self::NORTH,
        }

        // Check Z
        match coord.z.cmp(&0) {
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Less => direction += Self::DOWN,
            std::cmp::Ordering::Greater => direction += Self::UP,
        }

        direction
    }

    /// Rotates a [`Direction`] counter clockwise by one step:
    ///
    /// `NorthEast` becomes `North`
    ///
    /// `North` becomes `NorthWest`
    ///
    /// etc...
    pub fn left45(self) -> Self {
        let z = if self.has_up() {
            1
        } else if self.has_down() {
            -1
        } else {
            0
        };

        let (x, y) = if self.has_north() {
            if self.has_east() {
                (0, 1) // NorthEast -> North
            } else if self.has_west() {
                (-1, 0) // NorthWest -> West
            } else {
                (-1, 1) // North -> NorthWest
            }
        } else if self.has_south() {
            if self.has_east() {
                (1, 0) // SouthEast -> East
            } else if self.has_west() {
                (0, -1) // SouthWest -> South
            } else {
                (1, -1) // South -> SouthEast
            }
        } else if self.has_east() {
            (1, 1) // East -> NorthEast
        } else if self.has_west() {
            (-1, -1) // West -> SouthWest
        } else {
            (0, 0) // Direction::{None, Up, Down}
        };

        Self::from_ivec3(IVec3::new(x, y, z))
    }

    /// Rotates a [`Direction`] counter clockwise by two steps:
    ///
    /// `NorthEast` becomes `NorthWest`
    ///
    /// `North` becomes `West`
    ///
    /// etc...
    pub fn left90(self) -> Self {
        let z = if self.has_up() {
            1
        } else if self.has_down() {
            -1
        } else {
            0
        };

        let (x, y) = if self.has_north() {
            if self.has_east() {
                (-1, 1) // NorthEast -> NorthWest
            } else if self.has_west() {
                (-1, -1) // NorthWest -> SouthWest
            } else {
                (-1, 0) // North -> West
            }
        } else if self.has_south() {
            if self.has_east() {
                (1, 1) // SouthEast -> NorthEast
            } else if self.has_west() {
                (1, -1) // SouthWest -> SouthEast
            } else {
                (1, 0) // South -> East
            }
        } else if self.has_east() {
            (0, 1) // East -> North
        } else if self.has_west() {
            (0, -1) // West -> South
        } else {
            (0, 0) // Direction::{None, Up, Down}
        };

        Self::from_ivec3(IVec3::new(x, y, z))
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    /// Rotates a [`Direction`] counter clockwise by three steps:
    ///
    /// `NorthEast` becomes `West`
    ///
    /// `North` becomes `SouthWest`
    ///
    /// etc...
    pub fn left135(self) -> Self {
        self.left90().left45()
    }

    /// Retrieves the opposite (rotated by four steps) [`Direction`] from another [`Direction`]
    /// where:
    ///
    /// `North` and `South` are opposite:
    ///
    /// `East` and `West` are opposite:
    ///
    /// `Up` and `Down` are opposite:
    pub fn opposite(self) -> Self {
        let x = if self.has_east() {
            -1
        } else {
            i32::from(self.has_west())
        };

        let y = if self.has_north() {
            -1
        } else {
            i32::from(self.has_south())
        };

        let z = if self.has_up() {
            -1
        } else {
            i32::from(self.has_down())
        };

        Self::from_ivec3(IVec3::new(x, y, z))
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    /// Rotates a [`Direction`] clockwise by three steps:
    ///
    /// `NorthEast` becomes `South`
    ///
    /// `North` becomes `SouthEast`
    ///
    /// etc...
    pub fn right135(self) -> Self {
        self.right90().right45()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    /// Rotates a [`Direction`] clockwise by two steps:
    ///
    /// `NorthEast` becomes `SouthEast`
    ///
    /// `North` becomes `East`
    ///
    /// etc...
    pub fn right90(self) -> Self {
        self.left90().left90().left90()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    /// Rotates a [`Direction`] clockwise by one step:
    ///
    /// `NorthEast` becomes `East`
    ///
    /// `North` becomes `NorthEast`
    ///
    /// etc...
    pub fn right45(self) -> Self {
        self.right90().left45()
    }

    /// Checks a [`Direction`] to determine if it is marked as `North`
    ///
    /// Returns: `true` if [`Direction`] is marked `North`
    pub const fn has_north(self) -> bool {
        self.0 & Self::NORTH.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `South`
    ///
    /// Returns: `true` if [`Direction`] is marked `South`
    pub const fn has_south(self) -> bool {
        self.0 & Self::SOUTH.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `East`
    ///
    /// Returns: `true` if [`Direction`] is marked `East`
    pub const fn has_east(self) -> bool {
        self.0 & Self::EAST.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `West`
    ///
    /// Returns: `true` if [`Direction`] is marked `West`
    pub const fn has_west(self) -> bool {
        self.0 & Self::WEST.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `Up`
    ///
    /// Returns: `true` if [`Direction`] is marked `Up`
    pub const fn has_up(self) -> bool {
        self.0 & Self::UP.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `Down`
    ///
    /// Returns: `true` if [`Direction`] is marked `Down`
    pub const fn has_down(self) -> bool {
        self.0 & Self::DOWN.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is a [`CardinalDirection`]
    ///
    /// Returns: `true` if [`Direction`] contains a [`CardinalDirection`]
    /// NOTE: This function has no reguard for `Up` or `Down` therefore `Direction::UP_NORTH`
    /// will return `true`
    pub const fn is_cardinal(self) -> bool {
        (self.has_north() || self.has_south()) && !(self.has_east() || self.has_west())
            || (self.has_east() || self.has_west()) && !(self.has_north() || self.has_south())
    }

    /// Checks a [`Direction`] to determine if it is a [`OrdinalDirection`]
    ///
    /// Returns: `true` if [`Direction`] contains a [`OrdinalDirection`]
    /// NOTE: This function has no reguard for `Up` or `Down` therefore
    /// `Direction::UP_NORTH_EAST` will return `true`
    pub const fn is_ordinal(self) -> bool {
        (self.has_north() || self.has_south()) && (self.has_east() || self.has_west())
    }

    /// Creates an iterator over all `CardinalDirection`s, `OrdinalDirection`s, and
    /// `VerticalDirection`s
    pub const fn all_3d() -> DirectionIter {
        DirectionIter::all_3d()
    }

    // /// Creates a [`Direction`] from a [`Octant`]
    // pub fn from_octant(octant: Octant) -> Direction {
    //     // TODO: match on the range??
    //     match octant.0 {
    //         0 => Self::EAST,
    //         1 => Self::NORTH,
    //         2 => Self::NORTH,
    //         3 => Self::WEST,
    //         4 => Self::WEST,
    //         5 => Self::SOUTH,
    //         6 => Self::SOUTH,
    //         7 => Self::EAST,
    //         _ => unreachable!(),
    //     }
    // }
}

impl DirectionIterator for Direction {
    /// Creates an iterator over all `CardinalDirection`s and `OridinalDirection`s
    fn all() -> DirectionIter {
        DirectionIter::all_2d()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn append(old_string: String, next: &str, first: bool) -> String {
            if first {
                next.to_string()
            } else {
                format!("{old_string}, {next}")
            }
        }

        let mut s = String::new();
        let mut first = true;

        if self.has_north() {
            s = append(s, "NORTH", first);
            first = false;
        }

        if self.has_east() {
            s = append(s, "EAST", first);
            first = false;
        }

        if self.has_south() {
            s = append(s, "SOUTH", first);
            first = false;
        }

        if self.has_west() {
            s = append(s, "WEST", first);
            first = false;
        }

        if self.has_up() {
            s = append(s, "UP", first);
            first = false;
        }

        if self.has_down() {
            s = append(s, "DOWN", first);
            // first = false;
        }

        write!(f, "Direction({s})")
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<Self> for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 | *rhs)
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<Self> for Direction {
    fn add_assign(&mut self, rhs: Self) {
        self.0 |= *rhs;
    }
}

impl Sub<Self> for Direction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !*rhs)
    }
}

impl SubAssign<Self> for Direction {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 &= !*rhs
    }
}
