use crate::prelude::*;

pub trait PathProvider<T> {
    fn get_neighbors(&self, position: IVec2, pass_through_data: &mut T) -> Vec<IVec2>;

    fn cost(&self, _from_position: IVec2, _to_position: IVec2, _pass_through_data: &mut T) -> u32 { 1 }

    fn distance(&self, origin: IVec2, destination: IVec2) -> u32 { origin.distance(destination) }

    fn generate_successors(&self, p: IVec2, pass_through_data: &mut T) -> Vec<(IVec2, u32)> {
        let neighbors = self.get_neighbors(p, pass_through_data);
        let mut successors = Vec::with_capacity(neighbors.len());
        for neighbor in neighbors {
            successors.push((neighbor, self.cost(p, neighbor, pass_through_data)));
        }
        successors
    }
}
