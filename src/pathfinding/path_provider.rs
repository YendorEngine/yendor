use crate::prelude::*;

pub trait PathProvider<T> {
    fn get_neighbors(&self, position: UVec2, pass_through_data: &mut T) -> Vec<UVec2>;

    fn cost(&self, _from_position: UVec2, _to_position: UVec2, _pass_through_data: &mut T) -> u32 { 1 }

    fn distance(&self, origin: UVec2, destination: UVec2) -> u32 { origin.distance(destination) }

    fn generate_successors(&self, p: UVec2, pass_through_data: &mut T) -> Vec<(UVec2, u32)> {
        let neighbors = self.get_neighbors(p, pass_through_data);
        let mut successors = Vec::with_capacity(neighbors.len());
        for neighbor in neighbors {
            successors.push((neighbor, self.cost(p, neighbor, pass_through_data)));
        }
        successors
    }
}
