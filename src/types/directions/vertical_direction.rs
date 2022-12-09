use super::*;

pub struct  VerticalDirection;

impl VerticalDirection {
    pub const UP: Direction = Direction::UP;
    pub const DOWN: Direction = Direction::DOWN;

    pub fn all() -> DirectionIter { DirectionIter::vertical() }
}