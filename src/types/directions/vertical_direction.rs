use super::*;

pub struct  VerticalDirection;

impl VerticalDirection {
    pub const UP: Direction = Direction::UP;
    pub const DOWN: Direction = Direction::DOWN;

    pub const fn all() -> DirectionIter { DirectionIter::vertical() }
}