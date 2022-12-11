// Include Private
use super::*;

/// Cardinal Directions Include (`Left`, `Right`)
pub struct HorizontalDirection(pub(crate) Direction);

impl HorizontalDirection {
    /// Returns a [`Direction`] representing `Left`
    pub const LEFT: Direction = Direction::LEFT;
    /// Returns a [`Direction`] representing `Right`
    pub const RIGHT: Direction = Direction::RIGHT;
}

impl DirectionIterator for HorizontalDirection {
    /// Returns an iterator over the [`Direction`]s (`Left`, `Right`)
    fn all() -> DirectionIter { DirectionIter::cardinal() }
}
