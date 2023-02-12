use std::slice;

use crate::prelude::*;

pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridChunks<'a, T> = slice::Chunks<'a, T>;
pub type GridChunksMut<'a, T> = slice::ChunksMut<'a, T>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid3d<T, const LAYER_COUNT: usize> {
    dimensions: UVec2,
    layers: [Grid<T>; LAYER_COUNT],
}

impl<T, const LAYER_COUNT: usize> Grid3d<T, LAYER_COUNT> {
    #[inline(always)]
    pub fn new_clone(dimensions: UVec2, value: T) -> Self
    where
        T: Clone,
    {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_clone(dimensions, value.clone());
            layers.push(layer);
        }

        Self {
            dimensions,
            layers: layers
                .try_into()
                .unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    #[inline(always)]
    pub fn new_copy(dimensions: UVec2, value: T) -> Self
    where
        T: Copy,
    {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_copy(dimensions, value);
            layers.push(layer);
        }

        Self {
            dimensions,
            layers: layers
                .try_into()
                .unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    #[inline(always)]
    pub fn new_default(dimensions: UVec2) -> Self
    where
        T: Default,
    {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_default(dimensions);
            layers.push(layer);
        }

        Self {
            dimensions,
            layers: layers
                .try_into()
                .unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    #[inline(always)]
    pub fn new_fn(dimensions: UVec2, f: impl Fn((usize, IVec2)) -> T) -> Self {
        let count = dimensions.size();
        let mut layers = Vec::new();
        for index in 0..LAYER_COUNT {
            let mut cells = Vec::with_capacity(count);
            for coord in dimensions.iter() {
                cells.push(f((index, coord)));
            }
            layers.push(Grid::<T> { cells, dimensions });
        }

        Self {
            dimensions,
            layers: layers
                .try_into()
                .unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    #[inline(always)]
    pub fn blit_clone<ToLayerId: Into<usize>, FromLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: UVec2,
        source: &Self,
        from_layer_id: FromLayerId,
        from: UVec2,
        dimensions: UVec2,
    ) where
        T: Clone,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            if let Some(from_layer) = source.get_grid_by_layer(from_layer_id) {
                to_layer.blit_clone(to, from_layer, from, dimensions);
            }
        }
    }

    #[inline(always)]
    pub fn blit_copy<ToLayerId: Into<usize>, FromLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: UVec2,
        source: &Self,
        from_layer_id: FromLayerId,
        from: UVec2,
        dimensions: UVec2,
    ) where
        T: Copy,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            if let Some(from_layer) = source.get_grid_by_layer(from_layer_id) {
                to_layer.blit_copy(to, from_layer, from, dimensions);
            }
        }
    }

    #[inline(always)]
    pub fn blit_clone_from_2d<ToLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: UVec2,
        source: &Grid<T>,
        from: UVec2,
        dimensions: UVec2,
    ) where
        T: Clone,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            to_layer.blit_clone(to, source, from, dimensions);
        }
    }

    #[inline(always)]
    pub fn blit_copy_from_2d<ToLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: UVec2,
        source: &Grid<T>,
        from: UVec2,
        dimensions: UVec2,
    ) where
        T: Copy,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            to_layer.blit_copy(to, source, from, dimensions);
        }
    }

    #[inline(always)]
    pub fn blit_clone_to_2d<FromLayerId: Into<usize>>(
        &self,
        from_layer_id: FromLayerId,
        from: UVec2,
        dest: &mut Grid<T>,
        to: UVec2,
        dimensions: UVec2,
    ) where
        T: Clone,
    {
        if let Some(from_layer) = self.get_grid_by_layer(from_layer_id) {
            dest.blit_clone(to, from_layer, from, dimensions);
        }
    }

    #[inline(always)]
    pub fn blit_copy_to_2d<FromLayerId: Into<usize>>(
        &self,
        from_layer_id: FromLayerId,
        from: UVec2,
        dest: &mut Grid<T>,
        to: UVec2,
        dimensions: UVec2,
    ) where
        T: Copy,
    {
        if let Some(from_layer) = self.get_grid_by_layer(from_layer_id) {
            dest.blit_copy(to, from_layer, from, dimensions);
        }
    }

    #[inline]
    pub const fn width(&self) -> u32 {
        self.dimensions.x
    }

    #[inline]
    pub const fn height(&self) -> u32 {
        self.dimensions.y
    }

    #[inline]
    pub const fn dimensions(&self) -> UVec2 {
        self.dimensions
    }

    #[inline]
    pub fn in_bounds(&self, pos: IVec2) -> bool {
        pos.is_valid(self.dimensions())
    }

    #[inline]
    pub fn is_layer<LayerId: Into<usize>>(&self, layer_id: LayerId) -> bool {
        layer_id.into() < self.layers.len()
    }

    #[inline]
    pub fn get_idx(&self, pos: IVec2) -> Option<usize> {
        if pos.is_valid(self.dimensions()) {
            Some(self.get_idx_unchecked(pos))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_idx_unchecked(&self, point: IVec2) -> usize {
        point.as_index_unchecked(self.width())
    }

    #[inline]
    pub fn index_to_pt(&self, idx: usize) -> Option<UVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.dimensions()) {
            Some(pt)
        } else {
            None
        }
    }

    #[inline]
    pub const fn index_to_pt_unchecked(&self, idx: usize) -> UVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        UVec2::new(x as u32, y as u32)
    }

    #[inline]
    pub fn get<LayerId: Into<usize>>(&self, layer_id: LayerId, index: UVec2) -> Option<&T> {
        self.get_grid_by_layer(layer_id)
            .and_then(|layer| layer.get(index))
    }

    #[inline]
    pub fn get_mut<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: UVec2,
    ) -> Option<&mut T> {
        self.get_grid_by_layer_mut(layer_id)
            .and_then(|layer| layer.get_mut(index))
    }

    #[inline]
    pub fn get_unchecked<LayerId: Into<usize>>(&self, layer_id: LayerId, index: UVec2) -> &T {
        self.layers[layer_id.into()].get_unchecked(index)
    }

    #[inline]
    pub fn get_mut_unchecked<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: UVec2,
    ) -> &mut T {
        self.layers[layer_id.into()].get_mut_unchecked(index)
    }

    #[inline]
    pub fn set<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: UVec2,
        value: T,
    ) -> Option<T> {
        self.get_grid_by_layer_mut(layer_id)
            .and_then(|layer| layer.set(index, value))
    }

    #[inline]
    pub fn set_unchecked<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: UVec2,
        value: T,
    ) -> T {
        self.layers[layer_id.into()].set_unchecked(index, value)
    }

    pub fn get_grid_by_layer<LayerId: Into<usize>>(&self, layer_id: LayerId) -> Option<&Grid<T>> {
        let layer_id = layer_id.into();
        if self.is_layer(layer_id) {
            Some(&self.layers[layer_id])
        } else {
            None
        }
    }

    pub fn get_grid_by_layer_mut<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
    ) -> Option<&mut Grid<T>> {
        let layer_id = layer_id.into();
        if self.is_layer(layer_id) {
            Some(&mut self.layers[layer_id])
        } else {
            None
        }
    }
}

impl<T, const LAYER_COUNT: usize> GridIterable<T> for Grid3d<T, LAYER_COUNT> {
    type IterChunkMutReturn<'a> = GridChunksMut<'a, Grid<T>> where T: 'a, Self: 'a;
    type IterChunkReturn<'a> = GridChunks<'a, Grid<T>> where T: 'a, Self: 'a;
    type IterMutReturn<'a> = GridIterMut<'a, Grid<T>> where T: 'a, Self: 'a;
    type IterReturn<'a> = GridIter<'a, Grid<T>> where T: 'a, Self: 'a;

    #[inline]
    fn iter(&self) -> Self::IterReturn<'_> {
        self.layers.iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMutReturn<'_> {
        self.layers.iter_mut()
    }

    #[inline]
    fn point_iter(&self) -> PointIterRowMajor {
        self.dimensions.iter()
    }

    #[inline]
    fn enumerate(&self) -> GridEnumerate<Self::IterReturn<'_>> {
        self.point_iter().zip(self.iter())
    }

    #[inline]
    fn rows(&self) -> Self::IterChunkReturn<'_> {
        self.layers.chunks(self.dimensions.x as usize)
    }

    #[inline]
    fn rows_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.layers.chunks_mut(self.dimensions.x as usize)
    }

    #[inline]
    fn cols(&self) -> Self::IterChunkReturn<'_> {
        self.layers.chunks(self.dimensions.x as usize)
    }

    #[inline]
    fn cols_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.layers.chunks_mut(self.dimensions.x as usize)
    }

    #[inline]
    fn iter_column(&self, x: usize) -> Option<GridIterCol<Self::IterReturn<'_>>> {
        if x < self.dimensions.size() {
            let w = self.width() as usize;
            return Some(self.layers[x..].iter().step_by(w));
        } else {
            None
        }
    }

    #[inline]
    fn iter_column_unchecked(&self, x: usize) -> GridIterCol<Self::IterReturn<'_>> {
        let w = self.width() as usize;
        return self.layers[x..].iter().step_by(w);
    }
}

//#########################################################################
// Serialize
//#########################################################################
#[cfg(feature = "serialize")]
impl<T: Serialize + Clone, const LAYER_COUNT: usize> Serialize for Grid3d<T, LAYER_COUNT> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = match serde::Serializer::serialize_struct(serializer, "Grid3d", 2) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        match serde::ser::SerializeStruct::serialize_field(
            &mut serde_state,
            "dimensions",
            &self.dimensions,
        ) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        match serde::ser::SerializeStruct::serialize_field(
            &mut serde_state,
            "layers",
            &self.layers.to_vec(),
        ) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        serde::ser::SerializeStruct::end(serde_state)
    }
}
