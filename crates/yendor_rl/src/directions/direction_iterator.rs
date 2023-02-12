// Include Private
use super::*;

/// An iterator over all [`Direction`]s
pub trait DirectionIterator {
    /// Returns an iterator over all [`Direction`]s
    fn all() -> DirectionIter;
}
