use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

// Include public
use crate::prelude::*;
// Include Private
use super::*;

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Direction(DirectionType);

impl Direction {
    pub const NONE: Self = Self(0);
    pub const NORTH: Self = Self(DirectionFlags::NORTH);
    pub const NORTH_EAST: Self = Self(DirectionFlags::NORTH | DirectionFlags::EAST);
    pub const EAST: Self = Self(DirectionFlags::EAST);
    pub const SOUTH_EAST: Self = Self(DirectionFlags::SOUTH | DirectionFlags::EAST);
    pub const SOUTH: Self = Self(DirectionFlags::SOUTH);
    pub const SOUTH_WEST: Self = Self(DirectionFlags::SOUTH | DirectionFlags::WEST);
    pub const WEST: Self = Self(DirectionFlags::WEST);
    pub const NORTH_WEST: Self = Self(DirectionFlags::NORTH | DirectionFlags::WEST);
    pub const UP: Self = Self(DirectionFlags::UP);
    pub const UP_NORTH: Self = Self(DirectionFlags::UP | DirectionFlags::NORTH);
    pub const UP_NORTH_EAST: Self =
        Self(DirectionFlags::UP | DirectionFlags::NORTH | DirectionFlags::EAST);
    pub const UP_EAST: Self = Self(DirectionFlags::UP | DirectionFlags::EAST);
    pub const UP_SOUTH_EAST: Self =
        Self(DirectionFlags::UP | DirectionFlags::SOUTH | DirectionFlags::EAST);
    pub const UP_SOUTH: Self = Self(DirectionFlags::UP | DirectionFlags::SOUTH);
    pub const UP_SOUTH_WEST: Self =
        Self(DirectionFlags::UP | DirectionFlags::SOUTH | DirectionFlags::WEST);
    pub const UP_WEST: Self = Self(DirectionFlags::UP | DirectionFlags::WEST);
    pub const UP_NORTH_WEST: Self =
        Self(DirectionFlags::UP | DirectionFlags::NORTH | DirectionFlags::WEST);
    pub const DOWN: Self = Self(DirectionFlags::DOWN);
    pub const DOWN_NORTH: Self = Self(DirectionFlags::DOWN | DirectionFlags::NORTH);
    pub const DOWN_NORTH_EAST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::NORTH | DirectionFlags::EAST);
    pub const DOWN_EAST: Self = Self(DirectionFlags::DOWN | DirectionFlags::EAST);
    pub const DOWN_SOUTH_EAST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::SOUTH | DirectionFlags::EAST);
    pub const DOWN_SOUTH: Self = Self(DirectionFlags::DOWN | DirectionFlags::SOUTH);
    pub const DOWN_SOUTH_WEST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::SOUTH | DirectionFlags::WEST);
    pub const DOWN_WEST: Self = Self(DirectionFlags::DOWN | DirectionFlags::WEST);
    pub const DOWN_NORTH_WEST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::NORTH | DirectionFlags::WEST);
}

impl Direction {
    // Bevy Transform uses N: 1, S: -1, E: 1, W: -1
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

    pub fn from_coord(coord: impl Point) -> Self {
        Self::from_ivec3(coord.as_ivec2().extend(0))
    }

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
    pub fn left135(self) -> Self {
        self.left90().left45()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    pub fn right45(self) -> Self {
        self.right90().left45()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    pub fn right90(self) -> Self {
        self.left90().left90().left90()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    pub fn right135(self) -> Self {
        self.right90().right45()
    }

    pub const fn has_north(self) -> bool {
        self.0 & Self::NORTH.0 != 0
    }

    pub const fn has_south(self) -> bool {
        self.0 & Self::SOUTH.0 != 0
    }

    pub const fn has_east(self) -> bool {
        self.0 & Self::EAST.0 != 0
    }

    pub const fn has_west(self) -> bool {
        self.0 & Self::WEST.0 != 0
    }

    pub const fn has_up(self) -> bool {
        self.0 & Self::UP.0 != 0
    }

    pub const fn has_down(self) -> bool {
        self.0 & Self::DOWN.0 != 0
    }

    /// Unaffected by Up / Down
    pub const fn is_cardinal(self) -> bool {
        !self.is_ordinal()
    }

    /// Unaffected by Up / Down
    pub const fn is_ordinal(self) -> bool {
        (self.has_north() || self.has_south()) && (self.has_east() || self.has_west())
    }

    pub fn all() -> DirectionIter {
        DirectionIter::all_2d()
    }
    pub fn all_3d() -> DirectionIter {
        DirectionIter::all_3d()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn append(old_string: String, next: &str, first: bool) -> String {
            if first {
                next.to_string()
            } else {
                format!("{}, {}", old_string, next)
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
            //first = false;
        }

        write!(f, "Direction({})", s)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<Direction> for Direction {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        Self(self.0 | *rhs)
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<Direction> for Direction {
    fn add_assign(&mut self, rhs: Direction) {
        self.0 |= *rhs;
    }
}

impl Sub<Direction> for Direction {
    type Output = Self;
    fn sub(self, rhs: Direction) -> Self::Output {
        Self(self.0 & !*rhs)
    }
}

impl SubAssign<Direction> for Direction {
    fn sub_assign(&mut self, rhs: Direction) {
        self.0 &= !*rhs
    }
}
