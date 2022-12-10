use crate::prelude::Direction;

use super::*;

pub(crate) const DIRECTION_TABLE: &[Direction; 26] = &[
    Direction::NORTH,
    Direction::EAST,
    Direction::SOUTH,
    Direction::WEST,
    Direction::NORTH_EAST,
    Direction::SOUTH_EAST,
    Direction::SOUTH_WEST,
    Direction::NORTH_WEST,
    Direction::UP,
    Direction::DOWN,
    Direction::UP_NORTH,
    Direction::UP_NORTH_EAST,
    Direction::UP_EAST,
    Direction::UP_SOUTH_EAST,
    Direction::UP_SOUTH,
    Direction::UP_SOUTH_WEST,
    Direction::UP_WEST,
    Direction::UP_NORTH_WEST,
    Direction::DOWN_NORTH,
    Direction::DOWN_NORTH_EAST,
    Direction::DOWN_EAST,
    Direction::DOWN_SOUTH_EAST,
    Direction::DOWN_SOUTH,
    Direction::DOWN_SOUTH_WEST,
    Direction::DOWN_WEST,
    Direction::DOWN_NORTH_WEST,
];
