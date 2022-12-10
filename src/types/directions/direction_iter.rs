use super::*;

pub struct DirectionIter {
    end: usize,
    current: usize,
}

impl DirectionIter {
    pub fn all_2d() -> Self {
        Self { current: 0, end: 8 }
    }

    pub fn all_3d() -> Self {
        Self {
            end: 26,
            current: 0,
        }
    }

    pub fn cardinal() -> Self {
        Self { current: 0, end: 4 }
    }

    pub fn ordinal() -> Self {
        Self { current: 4, end: 8 }
    }

    pub fn vertical() -> Self {
        Self {
            end: 10,
            current: 8,
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
            Some(DIRECTION_TABLE[next])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn iter_all_2d() {
        let table = [
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
            Direction::NORTH_EAST,
            Direction::SOUTH_EAST,
            Direction::SOUTH_WEST,
            Direction::NORTH_WEST,
        ];
        assert_eq!(DirectionIter::all_2d().count(), 8);
        assert_eq!(DirectionIter::all_2d().collect::<Vec<_>>(), table)
    }

    #[test]
    fn iter_all_3d() {
        let table = [
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
        assert_eq!(DirectionIter::all_3d().count(), 26);
        assert_eq!(DirectionIter::all_3d().collect::<Vec<_>>(), table)
    }

    #[test]
    fn iter_cardinal() {
        let table = [
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ];
        assert_eq!(DirectionIter::cardinal().count(), 4);
        assert_eq!(DirectionIter::cardinal().collect::<Vec<_>>(), table)
    }

    #[test]
    fn iter_ordinal() {
        let table = [
            Direction::NORTH_EAST,
            Direction::SOUTH_EAST,
            Direction::SOUTH_WEST,
            Direction::NORTH_WEST,
        ];
        assert_eq!(DirectionIter::ordinal().count(), 4);
        assert_eq!(DirectionIter::ordinal().collect::<Vec<_>>(), table)
    }

    #[test]
    fn iter_vertical() {
        let table = [Direction::UP, Direction::DOWN];
        assert_eq!(DirectionIter::vertical().count(), 2);
        assert_eq!(DirectionIter::vertical().collect::<Vec<_>>(), table)
    }
}
