use core::slice;
use std::ops::{Index, IndexMut};

use crate::prelude::*;

pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridChunks<'a, T> = slice::Chunks<'a, T>;
pub type GridChunksMut<'a, T> = slice::ChunksMut<'a, T>;

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Grid<T: GridParam, const DIMENSIONS: UVec2> {
    pub cells: Vec<T>,
}

impl<T: GridParam, const DIMENSIONS: UVec2> GridLayer<T, DIMENSIONS> for Grid<T, DIMENSIONS> {
    type MutableReturn<'a> = &'a mut T;

    #[inline(always)]
    fn new_clone(value: T) -> Self
    where T: Clone {
        let count = DIMENSIONS.len();
        let mut cells = Vec::with_capacity(count);
        cells.resize(count, value);
        Self { cells }
    }

    #[inline(always)]
    fn blit_clone(&mut self, to: impl Point, source: &Self, from: impl Point)
    where T: Clone {
        DIMENSIONS.iter().for_each(|coord| {
            let x = coord.x_uint32();
            let y = coord.y_uint32();
            if let Some(val) = source.get((x + from.x_uint32(), y + from.y_uint32())) {
                self.set((x + to.x_uint32(), y + to.y_uint32()), val.clone());
            }
        });
    }

    #[inline(always)]
    fn new_copy(value: T) -> Self
    where T: Copy {
        let count = DIMENSIONS.len();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, || value);
        Self { cells }
    }

    #[inline(always)]
    fn blit_copy(&mut self, to: impl Point, source: &Self, from: impl Point)
    where T: Copy {
        DIMENSIONS.iter().for_each(|coord| {
            let x = coord.x_uint32();
            let y = coord.y_uint32();
            if let Some(val) = source.get((x + from.x_uint32(), y + from.y_uint32())) {
                self.set((x + to.x_uint32(), y + to.y_uint32()), *val);
            }
        });
    }

    #[inline(always)]
    fn new_default() -> Self
    where T: Default {
        let count = DIMENSIONS.len();
        let mut cells = Vec::new();
        cells.resize_with(count, Default::default);
        Self { cells }
    }

    #[inline(always)]
    fn new_fn(f: impl Fn(IVec2) -> T) -> Self {
        let count = DIMENSIONS.len();
        let mut cells = Vec::with_capacity(count);
        DIMENSIONS.iter().for_each(|coord| cells.push(f(coord)));
        Self { cells }
    }

    #[inline]
    fn width(&self) -> u32 { DIMENSIONS.width() }

    #[inline]
    fn height(&self) -> u32 { DIMENSIONS.height() }

    #[inline]
    fn size(&self) -> UVec2 { DIMENSIONS }

    #[inline]
    fn len(&self) -> usize { self.cells.len() }

    #[inline]
    fn is_empty(&self) -> bool { self.cells.is_empty() }

    #[inline]
    fn in_bounds(&self, pos: impl Point) -> bool { pos.is_valid(self.size()) }

    #[inline]
    fn get_idx(&self, pos: impl Point) -> Option<usize> {
        if pos.is_valid(self.size()) {
            Some(self.get_idx_unchecked(pos))
        } else {
            None
        }
    }

    #[inline]
    fn get_idx_unchecked(&self, point: impl Point) -> usize { point.as_index_unchecked(self.width()) }

    #[inline]
    fn index_to_pt(&self, idx: usize) -> Option<IVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.size()) {
            Some(pt)
        } else {
            None
        }
    }

    #[inline]
    fn index_to_pt_unchecked(&self, idx: usize) -> IVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        IVec2::new(x as i32, y as i32)
    }

    #[inline]
    fn get(&self, index: impl Point) -> Option<&T> { self.get_idx(index).map(|idx| &self.cells[idx]) }

    #[inline]
    fn get_mut(&mut self, index: impl Point) -> Option<&mut T> {
        self.get_idx(index).map(move |idx| &mut self.cells[idx])
    }

    #[inline]
    fn get_unchecked(&self, index: impl Point) -> &T { self.cells.index(self.get_idx_unchecked(index)) }

    #[inline]
    fn get_mut_unchecked(&mut self, index: impl Point) -> &mut T {
        self.cells.index_mut(self.get_idx_unchecked(index))
    }

    #[inline]
    fn set(&mut self, index: impl Point, value: T) -> Option<T> {
        if index.is_valid(self.size()) {
            let index = self.get_idx_unchecked(index);
            Some(std::mem::replace(&mut self.cells[index], value))
        } else {
            None
        }
    }

    #[inline]
    fn set_unchecked(&mut self, index: impl Point, value: T) -> T {
        let index = self.get_idx_unchecked(index);
        std::mem::replace(&mut self.cells[index], value)
    }
}

impl<T: GridParam, const DIMENSIONS: UVec2> GridIterable<T> for Grid<T, DIMENSIONS> {
    type IterChunkMutReturn<'a> = GridChunksMut<'a, T>;
    type IterChunkReturn<'a> = GridChunks<'a, T>;
    type IterMutReturn<'a> = GridIterMut<'a, T>;
    type IterReturn<'a> = GridIter<'a, T>;

    #[inline]
    fn iter(&self) -> GridIter<T> { self.cells.iter() }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    fn iter_mut(&mut self) -> GridIterMut<T> { self.cells.iter_mut() }

    #[inline]
    fn point_iter(&self) -> PointIterRowMajor { DIMENSIONS.iter() }

    #[inline]
    fn enumerate(&self) -> GridEnumerate<Self::IterReturn<'_>> { self.point_iter().zip(self.iter()) }

    #[inline]
    fn rows(&self) -> Self::IterChunkReturn<'_> { self.cells.chunks(DIMENSIONS.width() as usize) }

    #[inline]
    fn rows_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(DIMENSIONS.width() as usize)
    }

    #[inline]
    fn cols(&self) -> Self::IterChunkReturn<'_> { self.cells.chunks(DIMENSIONS.width() as usize) }

    #[inline]
    fn cols_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(DIMENSIONS.width() as usize)
    }

    #[inline]
    fn iter_column(&self, x: usize) -> Option<GridIterCol<Self::IterReturn<'_>>> {
        if x < DIMENSIONS.len() {
            let w = self.width() as usize;
            return Some(self.cells[x..].iter().step_by(w));
        } else {
            None
        }
    }

    #[inline]
    fn iter_column_unchecked(&self, x: usize) -> GridIterCol<Self::IterReturn<'_>> {
        let w = self.width() as usize;
        return self.cells[x..].iter().step_by(w);
    }
}

///////////////////////////////////////////////////////////////////////////
// Deref/DerefMut
///////////////////////////////////////////////////////////////////////////
// Deref
impl<T: GridParam, const DIMENSIONS: UVec2> std::ops::Deref for Grid<T, DIMENSIONS> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target { &self.cells }
}

// DerefMut
impl<T: GridParam, const DIMENSIONS: UVec2> std::ops::DerefMut for Grid<T, DIMENSIONS> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.cells }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////
impl<T: Copy + GridParam, const DIMENSIONS: UVec2> Index<usize> for Grid<T, DIMENSIONS> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T { &self.cells[index] }
}
impl<T: Copy + GridParam, const DIMENSIONS: UVec2> std::ops::IndexMut<usize> for Grid<T, DIMENSIONS> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.cells[index] }
}

impl<T: Copy + GridParam, P: Point, const DIMENSIONS: UVec2> Index<P> for Grid<T, DIMENSIONS> {
    type Output = T;

    #[inline]
    fn index(&self, index: P) -> &T { self.get_unchecked(index) }
}

impl<T: Copy + GridParam, P: Point, const DIMENSIONS: UVec2> IndexMut<P> for Grid<T, DIMENSIONS> {
    #[inline]
    fn index_mut(&mut self, index: P) -> &mut Self::Output { self.get_mut_unchecked(index) }
}
