use crate::prelude::*;
pub trait PathProvider<T, const DIMENSIONS: UVec2> {
    fn get_neighbors(
        &self,
        position: Position<DIMENSIONS>,
        pass_through_data: &mut T,
    ) -> Vec<Position<DIMENSIONS>>;

    fn cost(&self, position: Position<DIMENSIONS>, pass_through_data: &mut T) -> u32;

    fn distance(&self, origin: Position<DIMENSIONS>, destination: Position<DIMENSIONS>) -> u32 {
        origin.distance(destination)
    }
}
