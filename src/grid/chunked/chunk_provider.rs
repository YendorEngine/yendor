use crate::prelude::*;

pub trait ChunkProvider<T> {
    fn store(&mut self, world_position: ChunkWorldPosition, cells: Vec<T>);
    fn load(&mut self, world_position: ChunkWorldPosition, dimensions: ChunkDimensions) -> Vec<T>;
}
