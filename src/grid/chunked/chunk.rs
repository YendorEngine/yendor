use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Chunk<T> {
    grid: Grid<T>,
}

// Constructor
//##################
impl<T> Chunk<T> {
    #[inline(always)]
    pub fn new(dimensions: UVec2, cells: Vec<T>) -> Self {
        let count = (dimensions.x * dimensions.y) as usize;
        if cells.len() != count {
            panic!("Dimensions({}) do not match cells({})", count, cells.len());
        }

        Self {
            grid: Grid::new(dimensions, cells),
        }
    }
}

// Getters
//##################
impl<T> Chunk<T> {
    #[inline]
    pub fn remove_cells(mut self) -> Vec<T> { self.grid.take() }
}

// Functions
//##################
impl<T> Chunk<T> {
    #[inline]
    pub fn get(&self, position: ChunkLocalPosition) -> &T {
        position.as_index(self.grid.dimensions());
        let index = (position.y * self.grid.width() + position.x) as usize;
        &self.grid[index]
    }

    #[inline]
    pub fn get_mut(&mut self, position: ChunkLocalPosition) -> &mut T {
        let index = (position.y * self.grid.width() + position.x) as usize;
        &mut self.grid[index]
    }

    #[inline]
    pub fn set(&mut self, position: ChunkLocalPosition, value: T) -> T {
        let index = (position.y * self.grid.width() + position.x) as usize;
        std::mem::replace(&mut self.grid[index], value)
    }
}

// Private
//##################
impl<T> Chunk<T> {}

// Implementations
//##################
impl<T> Chunk<T> {}
