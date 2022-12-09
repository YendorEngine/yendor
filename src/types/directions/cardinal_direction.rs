use super::*;

pub struct  CardinalDirection;

impl CardinalDirection {
    pub const NORTH: Direction = Direction::NORTH;
    pub const EAST: Direction = Direction::EAST;
    pub const SOUTH: Direction = Direction::SOUTH;
    pub const WEST: Direction = Direction::WEST;

    pub fn all() -> DirectionIter { DirectionIter::cardinal() }
}