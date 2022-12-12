use crate::prelude::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Grid3d<T: GridParam, const LAYER_COUNT: usize, const DIM: UVec2> {
    layers: [Grid<T, DIM>; LAYER_COUNT],
}

impl<T: GridParam, const LAYER_COUNT: usize, const DIM: UVec2> Grid3d<T, LAYER_COUNT, DIM> {
    pub fn new_clone(value: T) -> Self
    where T: Clone {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_clone(value.clone());
            layers.push(layer);
        }

        Self {
            layers: layers.try_into().unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    pub fn new_copy(value: T) -> Self
    where T: Copy {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_copy(value);
            layers.push(layer);
        }

        Self {
            layers: layers.try_into().unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    pub fn new_default() -> Self
    where T: Default {
        let mut layers = Vec::new();
        for _ in 0..LAYER_COUNT {
            let layer = Grid::new_default();
            layers.push(layer);
        }

        Self {
            layers: layers.try_into().unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    pub fn new_fn(f: impl Fn((usize, IVec2)) -> T) -> Self {
        let count = DIM.size();
        let mut layers = Vec::new();
        for index in 0..LAYER_COUNT {
            let mut cells = Vec::with_capacity(count);
            DIM.iter().for_each(|coord| cells.push(f((index, coord))));
            layers.push(Grid::<T, DIM> { cells });
        }

        Self {
            layers: layers.try_into().unwrap_or_else(|_v| panic!("Something went wrong!")),
        }
    }

    pub fn blit_clone<ToLayerId: Into<usize>, FromLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: impl Point,
        source: &Self,
        from_layer_id: FromLayerId,
        from: impl Point,
    ) where
        T: Clone,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            if let Some(from_layer) = source.get_grid_by_layer(from_layer_id) {
                to_layer.blit_clone(to, from_layer, from);
            }
        }
    }

    pub fn blit_copy<ToLayerId: Into<usize>, FromLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: impl Point,
        source: &Self,
        from_layer_id: FromLayerId,
        from: impl Point,
    ) where
        T: Copy,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            if let Some(from_layer) = source.get_grid_by_layer(from_layer_id) {
                to_layer.blit_copy(to, from_layer, from);
            }
        }
    }

    pub fn blit_clone_from_2d<ToLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: impl Point,
        source: &Grid<T, DIM>,
        from: impl Point,
    ) where
        T: Clone,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            to_layer.blit_clone(to, source, from);
        }
    }

    pub fn blit_copy_from_2d<ToLayerId: Into<usize>>(
        &mut self,
        to_layer_id: ToLayerId,
        to: impl Point,
        source: &Grid<T, DIM>,
        from: impl Point,
    ) where
        T: Copy,
    {
        if let Some(to_layer) = self.get_grid_by_layer_mut(to_layer_id) {
            to_layer.blit_copy(to, source, from);
        }
    }

    pub fn blit_clone_to_2d<FromLayerId: Into<usize>>(
        &self,
        from_layer_id: FromLayerId,
        from: impl Point,
        dest: &mut Grid<T, DIM>,
        to: impl Point,
    ) where
        T: Clone,
    {
        if let Some(from_layer) = self.get_grid_by_layer(from_layer_id) {
            dest.blit_clone(to, from_layer, from);
        }
    }

    pub fn blit_copy_to_2d<FromLayerId: Into<usize>>(
        &self,
        from_layer_id: FromLayerId,
        from: impl Point,
        dest: &mut Grid<T, DIM>,
        to: impl Point,
    ) where
        T: Copy,
    {
        if let Some(from_layer) = self.get_grid_by_layer(from_layer_id) {
            dest.blit_copy(to, from_layer, from);
        }
    }

    #[inline]
    pub fn width(&self) -> u32 { DIM.width() }

    #[inline]
    pub fn height(&self) -> u32 { DIM.height() }

    #[inline]
    pub const fn size(&self) -> UVec2 { DIM }

    #[inline]
    pub fn in_bounds(&self, pos: impl Point) -> bool { pos.is_valid(self.size()) }

    #[inline]
    pub fn is_layer<LayerId: Into<usize>>(&self, layer_id: LayerId) -> bool {
        layer_id.into() < self.layers.len()
    }

    #[inline]
    pub fn get_idx(&self, pos: impl Point) -> Option<usize> {
        if pos.is_valid(self.size()) { Some(self.get_idx_unchecked(pos)) } else { None }
    }

    #[inline]
    pub fn get_idx_unchecked(&self, point: impl Point) -> usize { point.as_index_unchecked(self.width()) }

    #[inline]
    pub fn index_to_pt(&self, idx: usize) -> Option<UVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.size()) { Some(pt) } else { None }
    }

    #[inline]
    pub fn index_to_pt_unchecked(&self, idx: usize) -> UVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        UVec2::new(x as u32, y as u32)
    }

    #[inline]
    pub fn get<LayerId: Into<usize>>(&self, layer_id: LayerId, index: impl Point) -> Option<&T> {
        self.get_grid_by_layer(layer_id).and_then(|layer| layer.get(index))
    }

    #[inline]
    pub fn get_mut<LayerId: Into<usize>>(&mut self, layer_id: LayerId, index: impl Point) -> Option<&mut T> {
        self.get_grid_by_layer_mut(layer_id).and_then(|layer| layer.get_mut(index))
    }

    #[inline]
    pub fn get_unchecked<LayerId: Into<usize>>(&self, layer_id: LayerId, index: impl Point) -> &T {
        self.layers[layer_id.into()].get_unchecked(index)
    }

    #[inline]
    pub fn get_mut_unchecked<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: impl Point,
    ) -> &mut T {
        self.layers[layer_id.into()].get_mut_unchecked(index)
    }

    #[inline]
    pub fn set<LayerId: Into<usize>>(&mut self, layer_id: LayerId, index: impl Point, value: T) -> Option<T> {
        self.get_grid_by_layer_mut(layer_id).and_then(|layer| layer.set(index, value))
    }

    #[inline]
    pub fn set_unchecked<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
        index: impl Point,
        value: T,
    ) -> T {
        self.layers[layer_id.into()].set_unchecked(index, value)
    }

    pub fn get_grid_by_layer<LayerId: Into<usize>>(&self, layer_id: LayerId) -> Option<&Grid<T, DIM>> {
        let layer_id = layer_id.into();
        if self.is_layer(layer_id) { Some(&self.layers[layer_id]) } else { None }
    }

    pub fn get_grid_by_layer_mut<LayerId: Into<usize>>(
        &mut self,
        layer_id: LayerId,
    ) -> Option<&mut Grid<T, DIM>> {
        let layer_id = layer_id.into();
        if self.is_layer(layer_id) { Some(&mut self.layers[layer_id]) } else { None }
    }
}

impl<T: GridParam, const LAYER_COUNT: usize, const DIM: UVec2> GridIterable<T>
    for Grid3d<T, LAYER_COUNT, DIM>
{
    type IterChunkMutReturn<'a> = GridChunksMut<'a, Grid<T, DIM>>;
    type IterChunkReturn<'a> = GridChunks<'a, Grid<T, DIM>>;
    type IterMutReturn<'a> = GridIterMut<'a, Grid<T, DIM>>;
    type IterReturn<'a> = GridIter<'a, Grid<T, DIM>>;

    #[inline]
    fn iter(&self) -> Self::IterReturn<'_> { self.layers.iter() }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMutReturn<'_> { self.layers.iter_mut() }

    #[inline]
    fn point_iter(&self) -> PointIterRowMajor { DIM.iter() }

    #[inline]
    fn enumerate(&self) -> GridEnumerate<Self::IterReturn<'_>> { self.point_iter().zip(self.iter()) }

    #[inline]
    fn rows(&self) -> Self::IterChunkReturn<'_> { self.layers.chunks(DIM.width() as usize) }

    #[inline]
    fn rows_mut(&mut self) -> Self::IterChunkMutReturn<'_> { self.layers.chunks_mut(DIM.width() as usize) }

    #[inline]
    fn cols(&self) -> Self::IterChunkReturn<'_> { self.layers.chunks(DIM.width() as usize) }

    #[inline]
    fn cols_mut(&mut self) -> Self::IterChunkMutReturn<'_> { self.layers.chunks_mut(DIM.width() as usize) }

    #[inline]
    fn iter_column(&self, x: usize) -> Option<GridIterCol<Self::IterReturn<'_>>> {
        if x < DIM.size() {
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
