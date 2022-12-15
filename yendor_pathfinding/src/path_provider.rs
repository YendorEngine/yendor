use crate::prelude::*;

pub trait PathProvider<T> {
    fn get_neighbors(&self, position: ChunkPosition, pass_through_data: &mut T) -> Vec<ChunkPosition>;

    fn cost(
        &self,
        _from_position: ChunkPosition,
        _to_position: ChunkPosition,
        _pass_through_data: &mut T,
    ) -> u32 {
        1
    }

    fn distance(&self, origin: ChunkPosition, destination: ChunkPosition) -> u32 {
        origin.distance(destination).unwrap()
    }

    fn generate_successors(&self, p: ChunkPosition, pass_through_data: &mut T) -> Vec<(ChunkPosition, u32)> {
        let neighbors = self.get_neighbors(p, pass_through_data);
        let mut successors = Vec::with_capacity(neighbors.len());
        for neighbor in neighbors {
            successors.push((neighbor, self.cost(p, neighbor, pass_through_data)));
        }
        successors
    }
}
