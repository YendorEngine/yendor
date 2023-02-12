// Include Private
use super::*;

/// Cardinal Directions Include (`NorthEast`, `SouthEast`, `SouthWest`, `NorthWest`)
pub struct OrdinalDirection(pub(crate) Direction);

impl OrdinalDirection {
    /// Returns a [`Direction`] representing `NorthEast`
    pub const NORTH_EAST: Direction = Direction::NORTH_EAST;
    /// Returns a [`Direction`] representing `NorthWest`
    pub const NORTH_WEST: Direction = Direction::NORTH_WEST;
    /// Returns a [`Direction`] representing `SouthEast`
    pub const SOUTH_EAST: Direction = Direction::SOUTH_EAST;
    /// Returns a [`Direction`] representing `SouthWest`
    pub const SOUTH_WEST: Direction = Direction::SOUTH_WEST;
}

impl DirectionIterator for OrdinalDirection {
    /// Returns an iterator over the [`Direction`]s (`NorthEast`, `SouthEast`, `SouthWest`,
    /// `NorthWest`)
    fn all() -> DirectionIter {
        DirectionIter::ordinal()
    }
}
