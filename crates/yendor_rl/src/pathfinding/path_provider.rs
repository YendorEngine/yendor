use crate::prelude::*;

/// Trait to implement path providers.
pub trait PathProvider<T> {
    /// Returns the neighbors of the specified position.
    fn get_neighbors(&self, position: IVec2, pass_through_data: &mut T) -> Vec<IVec2>;

    /// Returns the cost of moving from the origin to the destination.
    fn cost(&self, _from_position: IVec2, _to_position: IVec2, _pass_through_data: &mut T) -> u32 {
        1
    }

    /// Returns the distance between the origin and the destination.
    fn distance(&self, origin: IVec2, destination: IVec2) -> u32 {
        origin.distance(destination)
    }

    /// Returns the successors of the specified position.
    fn generate_successors(&self, p: IVec2, pass_through_data: &mut T) -> Vec<(IVec2, u32)> {
        let neighbors = self.get_neighbors(p, pass_through_data);
        let mut successors = Vec::with_capacity(neighbors.len());
        for neighbor in neighbors {
            successors.push((neighbor, self.cost(p, neighbor, pass_through_data)));
        }
        successors
    }
}
