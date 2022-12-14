use std::ops::Index;

use bitvec::{bitvec, prelude::Lsb0, ptr::BitRef, slice, vec::BitVec};

use crate::prelude::*;

pub type BitIter<'a> = slice::Iter<'a, usize, Lsb0>;
pub type BitIterMut<'a> = slice::IterMut<'a, usize, Lsb0>;
pub type BitChunk<'a> = slice::Chunks<'a, usize, Lsb0>;
pub type BitChunkMut<'a> = slice::ChunksMut<'a, usize, Lsb0>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct BitGrid {
    pub cells: BitVec,
    pub dimensions: UVec2,
}

impl GridLayer<bool> for BitGrid {
    type MutableReturn<'a> = BitRef<'a, bitvec::ptr::Mut>;

    #[inline(always)]
    fn new_clone(dimensions: impl Dimensions, value: bool) -> Self {
        let count = dimensions.size();
        let mut cells = BitVec::with_capacity(count);
        cells.resize(count, value);
        Self {
            cells,
            dimensions: dimensions.as_uvec2(),
        }
    }

    #[inline(always)]
    fn blit_clone(&mut self, to: impl Point, source: &Self, from: impl Point, dimensions: impl Dimensions) {
        for y in 0..dimensions.height() {
            for x in 0..dimensions.width() {
                if let Some(val) = source.get((x + from.x_uint32(), y + from.y_uint32())) {
                    self.set((x + to.x_uint32(), y + to.y_uint32()), *val);
                }
            }
        }
    }

    #[inline(always)]
    fn new_copy(dimensions: impl Dimensions, value: bool) -> Self {
        let count = dimensions.size();
        let mut cells = BitVec::with_capacity(count);
        cells.resize_with(count, |_| value);
        Self {
            cells,
            dimensions: dimensions.as_uvec2(),
        }
    }

    #[inline(always)]
    fn blit_copy(&mut self, to: impl Point, source: &Self, from: impl Point, dimensions: impl Dimensions) {
        for y in 0..dimensions.height() {
            for x in 0..dimensions.width() {
                if let Some(val) = source.get((x + from.x_uint32(), y + from.y_uint32())) {
                    self.set((x + to.x_uint32(), y + to.y_uint32()), *val);
                }
            }
        }
    }

    #[inline(always)]
    fn new_default(dimensions: impl Dimensions) -> Self {
        let count = dimensions.size();
        Self {
            cells: bitvec![0_usize; count],
            dimensions: dimensions.as_uvec2(),
        }
    }

    #[inline(always)]
    fn new_fn(dimensions: impl Dimensions, f: impl Fn(IVec2) -> bool) -> Self {
        let count = dimensions.size();
        let mut cells = BitVec::with_capacity(count);
        for coord in dimensions.iter() {
            cells.push(f(coord));
        }
        Self {
            cells,
            dimensions: dimensions.as_uvec2(),
        }
    }

    #[inline]
    fn width(&self) -> u32 { self.dimensions.width() }

    #[inline]
    fn height(&self) -> u32 { self.dimensions.height() }

    #[inline]
    fn dimensions(&self) -> UVec2 { self.dimensions }

    #[inline]
    fn len(&self) -> usize { self.cells.len() }

    #[inline]
    fn is_empty(&self) -> bool { self.cells.is_empty() }

    #[inline]
    fn in_bounds(&self, point: impl Point) -> bool { point.is_valid(self.dimensions) }

    #[inline]
    fn get_idx_unchecked(&self, point: impl Point) -> usize { point.as_index_unchecked(self.width()) }

    #[inline]
    fn get_idx(&self, coord: impl Point) -> Option<usize> {
        if coord.is_valid(self.dimensions) { Some(self.get_idx_unchecked(coord)) } else { None }
    }

    #[inline]
    fn index_to_pt_unchecked(&self, idx: usize) -> IVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        IVec2::new(x as i32, y as i32)
    }

    #[inline]
    fn index_to_pt(&self, idx: usize) -> Option<IVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.dimensions) { Some(pt) } else { None }
    }

    #[inline]
    fn get(&self, pos: impl Point) -> Option<&bool> { self.get_idx(pos).map(|idx| &self.cells[idx]) }

    fn get_mut(&mut self, pos: impl Point) -> Option<Self::MutableReturn<'_>> {
        let w = self.width();
        self.cells.get_mut(pos.as_index_unchecked(w))
    }

    fn get_unchecked(&self, pos: impl Point) -> &bool { self.cells.index(self.get_idx_unchecked(pos)) }

    /// Gets a mutable reference corresponding to an index
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check if the index is out of bounds.
    fn get_mut_unchecked(&mut self, pos: impl Point) -> Self::MutableReturn<'_> {
        let w = self.width();
        unsafe { self.cells.get_unchecked_mut(pos.as_index_unchecked(w)) }
    }

    fn set(&mut self, pos: impl Point, value: bool) -> Option<bool> {
        if pos.is_valid(self.dimensions) {
            let index = self.get_idx_unchecked(pos);
            Some(self.cells.replace(index, value))
        } else {
            None
        }
    }

    fn set_unchecked(&mut self, pos: impl Point, value: bool) -> bool {
        let index = self.get_idx_unchecked(pos);
        self.cells.replace(index, value)
    }
}

impl GridIterable<bool> for BitGrid {
    type IterChunkMutReturn<'a> = BitChunkMut<'a>;
    type IterChunkReturn<'a> = BitChunk<'a>;
    type IterMutReturn<'a> = BitIterMut<'a>;
    type IterReturn<'a> = BitIter<'a>;

    #[inline]
    fn iter(&self) -> Self::IterReturn<'_> { self.cells.iter() }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMutReturn<'_> { self.cells.iter_mut() }

    #[inline]
    fn point_iter(&self) -> PointIterRowMajor { self.dimensions.iter() }

    #[inline]
    fn enumerate(&self) -> GridEnumerate<Self::IterReturn<'_>> { self.point_iter().zip(self.iter()) }

    #[inline]
    fn rows(&self) -> Self::IterChunkReturn<'_> { self.cells.chunks(self.dimensions.width() as usize) }

    #[inline]
    fn rows_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(self.dimensions.width() as usize)
    }

    #[inline]
    fn cols(&self) -> Self::IterChunkReturn<'_> { self.cells.chunks(self.dimensions.width() as usize) }

    #[inline]
    fn cols_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(self.dimensions.width() as usize)
    }

    #[inline]
    fn iter_column(&self, x: usize) -> Option<GridIterCol<Self::IterReturn<'_>>> {
        if x < self.dimensions.size() {
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
// Indexing
///////////////////////////////////////////////////////////////////////////

impl Index<usize> for BitGrid {
    type Output = bool;

    #[inline]
    fn index(&self, index: usize) -> &bool { &self.cells[index] }
}
