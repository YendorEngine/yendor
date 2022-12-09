use super::*;

/// Cardinal Directions Include (`North`, `East`, `South`, `West`)
pub struct  CardinalDirection;

impl CardinalDirection {
    /// Returns a [`Direction`] representing `North`
    pub const NORTH: Direction = Direction::NORTH;
    /// Returns a [`Direction`] representing `East`
    pub const EAST: Direction = Direction::EAST;
    /// Returns a [`Direction`] representing `South`
    pub const SOUTH: Direction = Direction::SOUTH;
    /// Returns a [`Direction`] representing `West`
    pub const WEST: Direction = Direction::WEST;

    /// Returns an iterator over the [`Direction`]s (`North`, `East`, `South`, `West`)
    pub fn all() -> DirectionIter { DirectionIter::cardinal() }
}