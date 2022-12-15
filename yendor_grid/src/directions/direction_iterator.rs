// Include Private
use super::*;

pub trait DirectionIterator {
    /// Returns an iterator over all [`Direction`]s
    fn all() -> DirectionIter;
}
