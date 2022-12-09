use super::*;

pub struct  OrdinalDirection;

impl OrdinalDirection {
    pub const NORTH_EAST: Direction = Direction::NORTH_EAST;
    pub const SOUTH_EAST: Direction = Direction::SOUTH_EAST;
    pub const SOUTH_WEST: Direction = Direction::SOUTH_WEST;
    pub const NORTH_WEST: Direction = Direction::NORTH_WEST;

    pub fn all() -> DirectionIter { DirectionIter::ordinal() }
}