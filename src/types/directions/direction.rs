use std::{ops::{Add, AddAssign, Sub, SubAssign}, fmt::Display};

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
    pub const UP_NORTH_EAST: Self = Self(DirectionFlags::UP | DirectionFlags::NORTH | DirectionFlags::EAST);
    pub const UP_EAST: Self = Self(DirectionFlags::UP | DirectionFlags::EAST);
    pub const UP_SOUTH_EAST: Self = Self(DirectionFlags::UP | DirectionFlags::SOUTH | DirectionFlags::EAST);
    pub const UP_SOUTH: Self = Self(DirectionFlags::UP | DirectionFlags::SOUTH);
    pub const UP_SOUTH_WEST: Self = Self(DirectionFlags::UP | DirectionFlags::SOUTH | DirectionFlags::WEST);
    pub const UP_WEST: Self = Self(DirectionFlags::UP | DirectionFlags::WEST);
    pub const UP_NORTH_WEST: Self = Self(DirectionFlags::UP | DirectionFlags::NORTH | DirectionFlags::WEST);
    pub const DOWN: Self = Self(DirectionFlags::DOWN);
    pub const DOWN_NORTH: Self = Self(DirectionFlags::DOWN | DirectionFlags::NORTH);
    pub const DOWN_NORTH_EAST: Self = Self(DirectionFlags::DOWN | DirectionFlags::NORTH | DirectionFlags::EAST);
    pub const DOWN_EAST: Self = Self(DirectionFlags::DOWN | DirectionFlags::EAST);
    pub const DOWN_SOUTH_EAST: Self = Self(DirectionFlags::DOWN | DirectionFlags::SOUTH | DirectionFlags::EAST);
    pub const DOWN_SOUTH: Self = Self(DirectionFlags::DOWN | DirectionFlags::SOUTH);
    pub const DOWN_SOUTH_WEST: Self = Self(DirectionFlags::DOWN | DirectionFlags::SOUTH | DirectionFlags::WEST);
    pub const DOWN_WEST: Self = Self(DirectionFlags::DOWN | DirectionFlags::WEST);
    pub const DOWN_NORTH_WEST: Self = Self(DirectionFlags::DOWN | DirectionFlags::NORTH | DirectionFlags::WEST);
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
            *direction |= *Self::EAST;
        } else if coord.y < 0 {
            *direction |= *Self::WEST;
        }

        if coord.y > 0 {
            *direction |= *Self::NORTH;
        } else if coord.y < 0 {
            *direction |= *Self::SOUTH;
        }

        if coord.z > 0 {
            *direction |= *Self::UP;
        } else if coord.z < 0 {
            *direction |= *Self::DOWN;
        }

        direction
    }

    pub const fn opposite(self) -> Self {
        let x = if *self & *Self::EAST != 0 {
            -1
        } else if *self & *Self::WEST != 0 {
            1
        } else {
            0
        };

        let y = if *self & *Self::NORTH != 0 {
            -1
        } else if *self & *Self::SOUTH != 0 {
            1
        } else {
            0
        };

        let z = if *self & *Self::UP != 0 {
            -1
        } else if *self & *Self::DOWN != 0 {
            1
        } else {
            0
        };

        Self::from_ivec3(IVec3::new(x, y ,z))
    }

    pub const fn left45(self) -> Self {
        let z = if self.has_up() {
            1
        } else if self.has_down() {
            -1
        } else {
            0
        };

        let (x, y) = if self.has_north() {
            if self.has_east() {
                (0, 1)  // NorthEast -> North
            } else if self.has_west() {
                (-1, 0) // NorthWest -> West
            } else {
                (-1, 1) // North -> NorthWest
            }
        } else if self.has_south() {
            if self.has_east() {
                (1, 0)  // SouthEast -> East
            } else if self.has_west() {
                (0, -1) // SouthWest -> South
            } else {
                (1, -1) // South -> SouthEast
            }
        } else if self.has_east() {
            (1, 1)      // East -> NorthEast
        } else if self.has_west() {
            (-1, -1)    // West -> SouthWest
        } else {
            (0, 0)      // Direction::{None, Up, Down}
        };

        Self::from_ivec3(IVec3::new(x, y, z))
    }

    pub const fn left90(self) -> Self {
        let z = if self.has_up() {
            1
        } else if self.has_down() {
            -1
        } else {
            0
        };

        let (x, y) = if self.has_north() {
            if self.has_east() {
                (-1, 1)  // NorthEast -> NorthWest
            } else if self.has_west() {
                (-1, -1) // NorthWest -> SouthWest
            } else {
                (-1, 0) // North -> West
            }
        } else if self.has_south() {
            if self.has_east() {
                (1, 1)  // SouthEast -> NorthEast
            } else if self.has_west() {
                (1, -1) // SouthWest -> SouthEast
            } else {
                (1, 0)  // South -> East
            }
        } else if self.has_east() {
            (0, 1)      // East -> North
        } else if self.has_west() {
            (0, -1)     // West -> South
        } else {
            (0, 0)      // Direction::{None, Up, Down}
        };

        Self::from_ivec3(IVec3::new(x, y, z))
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    pub const fn left135(self) -> Self {
        self.left90().left45()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    pub const fn right45(self) -> Self {
        self.right90().left45()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    pub const fn right90(self) -> Self {
        self.left90().left90().left90()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    pub const fn right135(self) -> Self {
        self.right90().right45()
    }

    pub fn has_north(self) -> bool {
        *self & *Self::NORTH != 0
    }

    pub fn has_south(self) -> bool {
        *self & *Self::SOUTH != 0
    }

    pub fn has_east(self) -> bool {
        *self & *Self::EAST != 0
    }

    pub fn has_west(self) -> bool {
        *self & *Self::WEST != 0
    }

    pub fn has_up(self) -> bool {
        *self & *Self::UP != 0
    }

    pub fn has_down(self) -> bool {
        *self & *Self::DOWN != 0
    }

    /// Unaffected by Up / Down
    pub const fn is_cardinal(self) -> bool {
        !self.is_ordinal()
    }
    
    /// Unaffected by Up / Down
    pub const fn is_ordinal(self) -> bool {
        (self.has_north() || self.has_south()) && (self.has_east() || self.has_west())
    }

    pub const fn all() -> DirectionIter { DirectionIter::all_2d() }
    pub const fn all_3d() -> DirectionIter { DirectionIter::all_3d() }
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
            first = false;
        }




        write!(f, "Direction({})", s)
    }
}

impl Add<Direction> for Direction {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        Self(*self | *rhs)
    }
}

impl AddAssign<Direction> for Direction {
    fn add_assign(&mut self, rhs: Direction) {
        self.0 |= *rhs;
    }
}

impl Sub<Direction> for Direction {
    type Output = Self;
    fn sub(self, rhs: Direction) -> Self::Output {
        Self(*self & !*rhs)
    }
}

impl SubAssign<Direction> for Direction {
    fn sub_assign(&mut self, rhs: Direction) {
        self.0 &= !*rhs
    }
}
