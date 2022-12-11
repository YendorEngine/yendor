use std::iter;

use crate::prelude::*;

pub type GridIterCol<'a, T> = iter::StepBy<T>;
pub type GridEnumerate<'a, T> = iter::Zip<PointIterRowMajor, T>;

pub trait GridIterable<T: GridParam> {
    /// iterator over the cells of the grid.
    type IterReturn<'a>
    where Self: 'a;

    /// mutable iterator over the cells of the grid.
    type IterMutReturn<'a>
    where Self: 'a;

    /// iterator over the row / col of the grid.
    type IterChunkReturn<'a>
    where Self: 'a;

    /// mutable iterator over the row / col of the grid.
    type IterChunkMutReturn<'a>
    where Self: 'a;

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    fn iter(&self) -> Self::IterReturn<'_>;

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    fn iter_mut(&mut self) -> Self::IterMutReturn<'_>;

    /// Returns an iterator for every point in the grid.
    fn point_iter(&self) -> PointIterRowMajor;

    /// Returns an iterator for every point in the grid with its corresponding point index
    fn enumerate(&self) -> GridEnumerate<Self::IterReturn<'_>>;

    ///////////////////////////////////////////////////////////////////////////
    // Row / Column Iterators
    ///////////////////////////////////////////////////////////////////////////
    /// Returns an iterator over the rows of the grid.
    fn rows(&self) -> Self::IterChunkReturn<'_>;

    /// Returns a mutable iterator over the rows of the grid.
    fn rows_mut(&mut self) -> Self::IterChunkMutReturn<'_>;

    /// Returns an iterator over the columns of the grid.
    fn cols(&self) -> Self::IterChunkReturn<'_>;

    /// Returns a mutable iterator over the columns of the grid.
    fn cols_mut(&mut self) -> Self::IterChunkMutReturn<'_>;

    /// Returns an iterator over a column of the grid.
    fn iter_column(&self, x: usize) -> Option<GridIterCol<Self::IterReturn<'_>>>;

    /// Returns a mutable iterator over a column of the grid.
    fn iter_column_unchecked(&self, x: usize) -> GridIterCol<Self::IterReturn<'_>>;
}
