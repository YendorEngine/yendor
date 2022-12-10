// Include Private
use super::*;

pub(crate) trait DirectionIterator {
    /// Returns an iterator over all [`Direction`]s
    fn all() -> DirectionIter;
}
