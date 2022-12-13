use crate::prelude::*;

pub trait PathProvider<T> {
    fn get_neighbors<const DIM: UVec2>(
        &self,
        position: Position<DIM>,
        pass_through_data: &mut T,
    ) -> Vec<Position<DIM>>;

    fn cost<const DIM: UVec2>(
        &self,
        _from_position: Position<DIM>,
        _to_position: Position<DIM>,
        _pass_through_data: &mut T,
    ) -> u32 {
        1
    }

    fn distance<const DIM: UVec2>(&self, origin: Position<DIM>, destination: Position<DIM>) -> u32 {
        origin.distance(destination)
    }

    fn generate_successors<const DIM: UVec2>(
        &self,
        p: Position<DIM>,
        pass_through_data: &mut T,
    ) -> Vec<(Position<DIM>, u32)> {
        let neighbors = self.get_neighbors(p, pass_through_data);
        let mut successors = Vec::with_capacity(neighbors.len());
        for neighbor in neighbors {
            successors.push((neighbor, self.cost(p, neighbor, pass_through_data)));
        }
        successors
    }
}
