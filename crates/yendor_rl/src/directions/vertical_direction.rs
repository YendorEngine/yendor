use super::*;

/// Cardinal Directions Include (`Up`, `Down`)
pub struct VerticalDirection;

impl VerticalDirection {
    /// Returns a [`Direction`] representing `Down`
    pub const DOWN: Direction = Direction::DOWN;
    /// Returns a [`Direction`] representing `Up`
    pub const UP: Direction = Direction::UP;

    /// Returns an iterator over the [`Direction`]s (`Up`, `Down`)
    pub const fn all() -> DirectionIter {
        DirectionIter::vertical()
    }
}
