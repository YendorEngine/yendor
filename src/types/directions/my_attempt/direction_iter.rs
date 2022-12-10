use super::*;

pub(crate) const DirectionTable: &[Direction; 27] = &[
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


pub struct DirectionIter {
    current: usize,
    end: usize,
}

impl DirectionIter {
    pub fn all_2d() -> Self {
        Self {
            current: 0,
            end: 7,
        }
    }

    pub fn all_3d() -> Self {
        Self {
            current: 0,
            end: 26,
        }
    }

    pub fn cardinal() -> Self {
        Self {
            current: 0,
            end: 3
        }
    }

    pub fn ordinal() -> Self {
        Self {
            current: 4,
            end: 7,
        }
    }

    pub fn vertical() -> Self {
        Self {
            current: 8,
            end: 9,
        }
    }
}


impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current;
        self.current += 1;
        if self.current > self.end {
            None
        } else {
            Some(DirectionTable[next])
        }
    }
}