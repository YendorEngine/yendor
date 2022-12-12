// Include Private
use super::*;

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
}

impl DirectionIterator for CardinalDirection {
    /// Returns an iterator over the [`Direction`]s (`NORTH`, `EAST`, `SOUTH`, `WEST`)
    fn all() -> DirectionIter { DirectionIter::cardinal() }
}
