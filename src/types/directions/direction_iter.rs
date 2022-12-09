use super::*;

// TODO: This needs some love....

pub struct DirectionIter {
    _directions: Vec<Direction>,
}

impl DirectionIter {
    pub fn all_2d() -> Self {
        Self { _directions: vec![
            Direction::NORTH,
            Direction::NORTH_EAST,
            Direction::EAST,
            Direction::SOUTH_EAST,
            Direction::SOUTH,
            Direction::SOUTH_WEST,
            Direction::WEST,
            Direction::NORTH_WEST,
        ] }
    }

    pub fn all_3d() -> Self {
        Self { _directions: vec![
            Direction::NORTH,
            Direction::NORTH_EAST,
            Direction::EAST,
            Direction::SOUTH_EAST,
            Direction::SOUTH,
            Direction::SOUTH_WEST,
            Direction::WEST,
            Direction::NORTH_WEST,
            Direction::UP,
            Direction::UP_NORTH,
            Direction::UP_NORTH_EAST,
            Direction::UP_EAST,
            Direction::UP_SOUTH_EAST,
            Direction::UP_SOUTH,
            Direction::UP_SOUTH_WEST,
            Direction::UP_WEST,
            Direction::UP_NORTH_WEST,
            Direction::DOWN,
            Direction::DOWN_NORTH,
            Direction::DOWN_NORTH_EAST,
            Direction::DOWN_EAST,
            Direction::DOWN_SOUTH_EAST,
            Direction::DOWN_SOUTH,
            Direction::DOWN_SOUTH_WEST,
            Direction::DOWN_WEST,
            Direction::DOWN_NORTH_WEST,
        ] }
    }

    pub fn cardinal() -> Self {
        Self { _directions: vec![
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ] }
    }
    
    pub fn ordinal() -> Self {
        Self { _directions: vec![
            Direction::NORTH_EAST,
            Direction::SOUTH_EAST,
            Direction::SOUTH_WEST,
            Direction::NORTH_WEST,
        ] }
    }

    pub fn vertical() -> Self {
        Self { _directions: vec![
            Direction::UP,
            Direction::DOWN,
        ] }
    }
}
