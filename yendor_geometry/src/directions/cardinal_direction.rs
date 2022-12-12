// Include Private
use super::*;
use crate::prelude::*;

/// Cardinal Directions Include (`NORTH`, `EAST`, `SOUTH`, `WEST`)
pub struct CardinalDirection;

impl CardinalDirection {
    /// Returns a [`Direction`] representing `EAST`
    pub const EAST: Direction = Direction::EAST;
    /// Returns a [`Direction`] representing `NORTH`
    pub const NORTH: Direction = Direction::NORTH;
    /// Returns a [`Direction`] representing `SOUTH`
    pub const SOUTH: Direction = Direction::SOUTH;
    /// Returns a [`Direction`] representing `WEST`
    pub const WEST: Direction = Direction::WEST;

    pub fn from_octant<const DIM: UVec2>(octant: Octant<DIM>) -> Direction {
        // TODO: match on the range??
        match octant.0 {
            0 => Self::EAST,
            1 => Self::NORTH,
            2 => Self::NORTH,
            3 => Self::WEST,
            4 => Self::WEST,
            5 => Self::SOUTH,
            6 => Self::SOUTH,
            7 => Self::EAST,
            _ => unreachable!(),
        }
    }
}

impl DirectionIterator for CardinalDirection {
    /// Returns an iterator over the [`Direction`]s (`NORTH`, `EAST`, `SOUTH`, `WEST`)
    fn all() -> DirectionIter { DirectionIter::cardinal() }
}
