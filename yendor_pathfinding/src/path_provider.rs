use crate::prelude::*;

pub trait PathProvider<T, const DIMENSIONS: UVec2> {
    fn get_neighbors(
        &self,
        position: Position<DIMENSIONS>,
        pass_through_data: &mut T,
    ) -> Vec<Position<DIMENSIONS>>;

    fn cost(
        &self,
        from_position: Position<DIMENSIONS>,
        to_position: Position<DIMENSIONS>,
        pass_through_data: &mut T,
    ) -> u32;

    fn distance(&self, origin: Position<DIMENSIONS>, destination: Position<DIMENSIONS>) -> u32 {
        origin.distance(destination)
    }

    fn generate_successors(
        &self,
        p: Position<DIMENSIONS>,
        pass_through_data: &mut T,
    ) -> Vec<(Position<DIMENSIONS>, u32)> {
        let neighbors = self.get_neighbors(p, pass_through_data);
        let mut successors = Vec::with_capacity(neighbors.len());
        for neighbor in neighbors {
            successors.push((neighbor, self.cost(p, neighbor, pass_through_data)));
        }
        successors
    }
}
